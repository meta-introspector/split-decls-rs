// Generated macro for abort_call_site (macro)
macro_rules! Depcrate_macrosabort_call_site {
() => {
// Module: crate::macros
// Provides: {"abort_call_site"}
// Dependencies: {}
macro_rules ! abort_call_site { ($ ($ format : tt) +) => { { let span = proc_macro2 :: Span :: call_site () ; abort ! (span , $ ($ format) +) } } ; }
};
}
