// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let m = Mutex :: new (5) ; { let mut num = m . lock () . unwrap () ; * num = 6 ; } println ! ("m = {m:?}") ; }
};
}
