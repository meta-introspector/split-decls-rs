// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { trpl :: block_on (async { let (tx , mut rx) = trpl :: channel () ; let vals = vec ! [String :: from ("hi") , String :: from ("from") , String :: from ("the") , String :: from ("future") ,] ; for val in vals { tx . send (val) . unwrap () ; trpl :: sleep (Duration :: from_millis (500)) . await ; } while let Some (value) = rx . recv () . await { println ! ("received '{value}'") ; } }) ; }
};
}
