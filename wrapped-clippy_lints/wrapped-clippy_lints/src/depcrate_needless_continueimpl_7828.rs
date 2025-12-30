// Generated macro for impl_7828 (impl)
macro_rules! Depcrate_needless_continueimpl_7828 {
() => {
// Module: crate::needless_continue
// Provides: {"impl_7828"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NeedlessContinue { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if ! matches ! (expr . span . ctxt () . outer_expn_data () . kind , ExpnKind :: Macro (..)) { check_and_warn (cx , expr) ; } } }
};
}
