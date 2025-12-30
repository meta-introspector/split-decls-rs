// Generated macro for late_lint_crate_inner (function)
macro_rules! Depcrate_latelate_lint_crate_inner {
() => {
// Module: crate::late
// Provides: {"late_lint_crate_inner"}
// Dependencies: {}
fn late_lint_crate_inner < 'tcx , T : LateLintPass < 'tcx > > (tcx : TyCtxt < 'tcx > , context : LateContext < 'tcx > , pass : T ,) { let mut cx = LateContextAndPass { context , pass } ; cx . with_lint_attrs (hir :: CRATE_HIR_ID , | cx | { lint_callback ! (cx , check_crate ,) ; tcx . hir_walk_toplevel_module (cx) ; lint_callback ! (cx , check_crate_post ,) ; }) }
};
}
