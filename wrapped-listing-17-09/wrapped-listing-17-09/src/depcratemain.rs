// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { trpl :: block_on (async { let (tx , mut rx) = trpl :: channel () ; let val = String :: from ("hi") ; tx . send (val) . unwrap () ; let received = rx . recv () . await . unwrap () ; println ! ("received '{received}'") ; }) ; }
};
}
