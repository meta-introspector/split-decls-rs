// Generated macro for impl_7944 (impl)
macro_rules! Depcrate_needless_parens_on_range_literalsimpl_7944 {
() => {
// Module: crate::needless_parens_on_range_literals
// Provides: {"impl_7944"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NeedlessParensOnRangeLiterals { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let Some (higher :: Range { start , end , .. }) = higher :: Range :: hir (cx , expr) { if let Some (start) = start { check_for_parens (cx , start , true) ; } if let Some (end) = end { check_for_parens (cx , end , false) ; } } } }
};
}
