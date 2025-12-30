// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let x = Some (5) ; let y = 10 ; match x { Some (50) => println ! ("Got 50") , Some (n) if n == y => println ! ("Matched, n = {n}") , _ => println ! ("Default case, x = {x:?}") , } println ! ("at the end: x = {x:?}, y = {y}") ; }
};
}
