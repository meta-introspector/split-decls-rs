// Generated macro for impl_2756 (impl)
macro_rules! Depcrate_ifsimpl_2756 {
() => {
// Module: crate::ifs
// Provides: {"impl_2756"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for CopyAndPaste < 'tcx > { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if ! expr . span . from_expansion () && matches ! (expr . kind , ExprKind :: If (..)) && ! is_else_clause (cx . tcx , expr) { let (conds , blocks) = if_sequence (expr) ; ifs_same_cond :: check (cx , & conds , & mut self . interior_mut) ; same_functions_in_if_cond :: check (cx , & conds) ; let all_same = ! is_lint_allowed (cx , IF_SAME_THEN_ELSE , expr . hir_id) && if_same_then_else :: check (cx , & conds , & blocks) ; if ! all_same && conds . len () != blocks . len () { branches_sharing_code :: check (cx , & conds , & blocks , expr) ; } } } }
};
}
