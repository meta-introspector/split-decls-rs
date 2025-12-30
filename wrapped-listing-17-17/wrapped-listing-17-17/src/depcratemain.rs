// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { trpl :: block_on (async { let a = async { println ! ("'a' started.") ; slow ("a" , 30) ; trpl :: yield_now () . await ; slow ("a" , 10) ; trpl :: yield_now () . await ; slow ("a" , 20) ; trpl :: yield_now () . await ; println ! ("'a' finished.") ; } ; let b = async { println ! ("'b' started.") ; slow ("b" , 75) ; trpl :: yield_now () . await ; slow ("b" , 10) ; trpl :: yield_now () . await ; slow ("b" , 15) ; trpl :: yield_now () . await ; slow ("b" , 350) ; trpl :: yield_now () . await ; println ! ("'b' finished.") ; } ; trpl :: select (a , b) . await ; }) ; }
};
}
