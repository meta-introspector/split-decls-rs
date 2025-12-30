// Generated macro for print_or_panic (macro)
macro_rules! Depcrate_runtimeprint_or_panic {
() => {
// Module: crate::runtime
// Provides: {"print_or_panic"}
// Dependencies: {}
# [cfg (feature = "glob")] macro_rules ! print_or_panic { ($ fail_fast : expr , $ ($ tokens : tt) *) => { { if (!$ fail_fast) { eprintln ! ($ ($ tokens) *) ; eprintln ! () ; } else { panic ! ($ ($ tokens) *) ; } } } }
};
}
