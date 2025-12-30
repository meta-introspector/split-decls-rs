// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let list = vec ! [1 , 2 , 3] ; println ! ("Before defining closure: {list:?}") ; thread :: spawn (move | | println ! ("From thread: {list:?}")) . join () . unwrap () ; }
};
}
