// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let (tx , mut rx) = trpl :: channel () ; thread :: spawn (move | | { for i in 1 .. 11 { tx . send (i) . unwrap () ; thread :: sleep (Duration :: from_secs (1)) ; } }) ; trpl :: block_on (async { while let Some (message) = rx . recv () . await { println ! ("{message}") ; } }) ; }
};
}
