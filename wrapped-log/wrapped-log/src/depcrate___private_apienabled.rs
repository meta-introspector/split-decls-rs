// Generated macro for enabled (function)
macro_rules! Depcrate___private_apienabled {
() => {
// Module: crate::__private_api
// Provides: {"enabled"}
// Dependencies: {}
pub fn enabled < L : Log > (logger : L , level : Level , target : & str) -> bool { logger . enabled (& Metadata :: builder () . level (level) . target (target) . build ()) }
};
}
