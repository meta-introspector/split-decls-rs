// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let counter = Mutex :: new (0) ; let mut handles = vec ! [] ; for _ in 0 .. 10 { let handle = thread :: spawn (move | | { let mut num = counter . lock () . unwrap () ; * num += 1 ; }) ; handles . push (handle) ; } for handle in handles { handle . join () . unwrap () ; } println ! ("Result: {}" , * counter . lock () . unwrap ()) ; }
};
}
