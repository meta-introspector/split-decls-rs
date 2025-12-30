// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let handle = thread :: spawn (| | { for i in 1 .. 10 { println ! ("hi number {i} from the spawned thread!") ; thread :: sleep (Duration :: from_millis (1)) ; } }) ; handle . join () . unwrap () ; for i in 1 .. 5 { println ! ("hi number {i} from the main thread!") ; thread :: sleep (Duration :: from_millis (1)) ; } }
};
}
