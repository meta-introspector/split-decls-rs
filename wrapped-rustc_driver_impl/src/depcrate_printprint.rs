// Generated macro for print (function)
macro_rules! Depcrate_printprint {
() => {
// Module: crate::print
// Provides: {"print"}
// Dependencies: {}
pub (crate) fn print (args : fmt :: Arguments < '_ >) { if let Err (_) = io :: stdout () . write_fmt (args) { rustc_errors :: FatalError . raise () ; } }
};
}
