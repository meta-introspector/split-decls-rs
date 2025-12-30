// Generated macro for impl_1267 (impl)
macro_rules! Depcrate_copiesimpl_1267 {
() => {
// Module: crate::copies
// Provides: {"impl_1267"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for CopyAndPaste < 'tcx > { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if ! expr . span . from_expansion () && matches ! (expr . kind , ExprKind :: If (..)) && ! is_else_clause (cx . tcx , expr) { let (conds , blocks) = if_sequence (expr) ; lint_same_cond (cx , & conds , & mut self . interior_mut) ; lint_same_fns_in_if_cond (cx , & conds) ; let all_same = ! is_lint_allowed (cx , IF_SAME_THEN_ELSE , expr . hir_id) && lint_if_same_then_else (cx , & conds , & blocks) ; if ! all_same && conds . len () != blocks . len () { lint_branches_sharing_code (cx , & conds , & blocks , expr) ; } } } }
};
}
