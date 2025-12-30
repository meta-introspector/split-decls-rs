// Generated macro for abort (macro)
macro_rules! Depcrate_macrosabort {
() => {
// Module: crate::macros
// Provides: {"abort"}
// Dependencies: {}
macro_rules ! abort { ($ obj : expr , $ ($ format : tt) +) => { { return Err (format_err ! ($ obj , $ ($ format) +)) ; } } ; }
};
}
