// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut counter = 0 ; let result = loop { counter += 1 ; if counter == 10 { break counter * 2 ; } } ; println ! ("The result is {result}") ; }
};
}
