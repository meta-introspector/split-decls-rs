// Generated macro for emit (function)
macro_rules! Depcrate_outputemit {
() => {
// Module: crate::output
// Provides: {"emit"}
// Dependencies: {}
fn emit (directive : & str , value : impl Display) { println ! ("cargo::{directive}={value}") ; }
};
}
