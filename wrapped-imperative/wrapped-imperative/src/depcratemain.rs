// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let pool = ThreadPool :: new () . expect ("Failed to build pool") ; let (tx , mut rx) = mpsc :: unbounded :: < i32 > () ; let fut_values = async { let fut_tx_result = async move { (0 .. 100) . for_each (| v | { tx . unbounded_send (v) . expect ("Failed to send") ; }) } ; pool . spawn_ok (fut_tx_result) ; let mut pending = vec ! [] ; while let Some (v) = rx . next () . await { pending . push (v * 2) ; } pending } ; let values : Vec < i32 > = executor :: block_on (fut_values) ; println ! ("Values={values:?}") ; }
};
}
