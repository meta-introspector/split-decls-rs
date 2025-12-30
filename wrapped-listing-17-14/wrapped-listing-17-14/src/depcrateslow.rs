// Generated macro for slow (function)
macro_rules! Depcrateslow {
() => {
// Module: crate
// Provides: {"slow"}
// Dependencies: {}
fn slow (name : & str , ms : u64) { thread :: sleep (Duration :: from_millis (ms)) ; println ! ("'{name}' ran for {ms}ms") ; }
};
}
