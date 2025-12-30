// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let guess = "3" ; let guess = match guess . trim () . parse () { Ok (_) => 5 , Err (_) => "hello" , } ; }
};
}
