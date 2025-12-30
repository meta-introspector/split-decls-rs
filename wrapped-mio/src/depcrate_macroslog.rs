// Generated macro for log (macro)
macro_rules! Depcrate_macroslog {
() => {
// Module: crate::macros
// Provides: {"log"}
// Dependencies: {}
macro_rules ! log { ($ level : ident , $ ($ t : tt) *) => { # [cfg (feature = "log")] { log ::$ level ! ($ ($ t) *) } # [cfg (not (feature = "log"))] { if false { let _ = ($ ($ t) *) ; } } } }
};
}
