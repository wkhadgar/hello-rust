fn main() {
    let notes = [10f32, 9.0, -7.0, 0.0];

    lazy_mean(&notes);
    nice_mean(&notes);
    pretty_mean(&notes);
}

fn lazy_mean(notes: &[f32]) -> f32 {
    let mut sum = 0.0;
    for note in notes {
        sum += note;
    }
    let result = sum / notes.len() as f32;
    println!("\n{}\n", result);

    result
}

fn nice_mean(notes: &[f32]) -> f32 {
    let mut i = 0;
    let mut sum = 0.0;
    print!("\n(");
    for note in notes {
        print!("{}", note);
        sum += note;

        i += 1;
        if i != notes.len() {
            print!(" + ");
        } else { print!(") / {} = ", notes.len()) }
    }
    let result = sum / notes.len() as f32;
    println!("{}\n", result);

    result
}

fn pretty_mean(notes: &[f32]) -> f32 {
    let mut i = 0;
    let mut sum = 0.0;
    print!("\n(");
    for note in notes {
        print!("{}", note.abs());
        sum += note;

        i += 1;
        if i != notes.len() {
            print!(" {} ", if notes[i] < 0.0 { '-' } else { '+' })
        } else { print!(")\n{} = ", "-".to_string().repeat(notes.len() * 4)) }
    }
    let result = sum / notes.len() as f32;
    println!("{}", result);

    println!("{}{}\n", " ".to_string().repeat(notes.len() * 2), notes.len());

    result
}