// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let (tx , rx) = mpsc :: channel () ; thread :: spawn (move | | { let val = String :: from ("hi") ; tx . send (val) . unwrap () ; println ! ("val is {val}") ; }) ; let received = rx . recv () . unwrap () ; println ! ("Got: {received}") ; }
};
}
