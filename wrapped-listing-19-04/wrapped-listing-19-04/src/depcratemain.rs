// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let (tx , rx) = std :: sync :: mpsc :: channel () ; std :: thread :: spawn (move | | { for val in [1 , 2 , 3] { tx . send (val) . unwrap () ; } }) ; while let Ok (value) = rx . recv () { println ! ("{value}") ; } }
};
}
