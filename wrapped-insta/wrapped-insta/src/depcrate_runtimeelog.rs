// Generated macro for elog (macro)
macro_rules! Depcrate_runtimeelog {
() => {
// Module: crate::runtime
// Provides: {"elog"}
// Dependencies: {}
# [macro_export] macro_rules ! elog { () => (write ! (std :: io :: stderr ()) . ok ()) ; ($ ($ arg : tt) *) => ({ writeln ! (std :: io :: stderr () , $ ($ arg) *) . ok () ; }) }
};
}
