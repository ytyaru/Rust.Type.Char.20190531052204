/*
 * Rustの型（文字）。
 * CreatedAt: 2019-05-31
 */
fn main() {
    let c1 = 'A';
    let c2: char = 'B';
//    let c3 = 'AB'; // error: character literal may only contain one codepoint
//    let c4: char = "A"; // error[E0308]: mismatched types
    println!("{} {}", c1, c2);
    if c1 == c2 { println!("c1 == c2"); }
    else { println!("c1 != c2"); }
}

