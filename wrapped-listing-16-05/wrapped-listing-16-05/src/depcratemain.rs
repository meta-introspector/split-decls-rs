// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let v = vec ! [1 , 2 , 3] ; let handle = thread :: spawn (move | | { println ! ("Here's a vector: {v:?}") ; }) ; handle . join () . unwrap () ; }
};
}
