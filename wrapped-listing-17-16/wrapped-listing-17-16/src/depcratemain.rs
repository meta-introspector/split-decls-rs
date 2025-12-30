// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { trpl :: block_on (async { let one_ms = Duration :: from_millis (1) ; let a = async { println ! ("'a' started.") ; slow ("a" , 30) ; trpl :: sleep (one_ms) . await ; slow ("a" , 10) ; trpl :: sleep (one_ms) . await ; slow ("a" , 20) ; trpl :: sleep (one_ms) . await ; println ! ("'a' finished.") ; } ; let b = async { println ! ("'b' started.") ; slow ("b" , 75) ; trpl :: sleep (one_ms) . await ; slow ("b" , 10) ; trpl :: sleep (one_ms) . await ; slow ("b" , 15) ; trpl :: sleep (one_ms) . await ; slow ("b" , 350) ; trpl :: sleep (one_ms) . await ; println ! ("'b' finished.") ; } ; trpl :: select (a , b) . await ; }) ; }
};
}
