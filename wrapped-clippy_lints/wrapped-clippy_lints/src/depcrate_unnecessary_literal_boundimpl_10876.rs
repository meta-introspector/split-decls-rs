// Generated macro for impl_10876 (impl)
macro_rules! Depcrate_unnecessary_literal_boundimpl_10876 {
() => {
// Module: crate::unnecessary_literal_bound
// Provides: {"impl_10876"}
// Dependencies: {}
impl < 'hir > Visitor < 'hir > for FindNonLiteralReturn { type Result = std :: ops :: ControlFlow < () > ; type NestedFilter = intravisit :: nested_filter :: None ; fn visit_expr (& mut self , expr : & 'hir Expr < 'hir >) -> Self :: Result { if let ExprKind :: Ret (Some (ret_val_expr)) = expr . kind && ! is_str_literal (ret_val_expr) { Self :: Result :: Break (()) } else { intravisit :: walk_expr (self , expr) } } }
};
}
