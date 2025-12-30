// Generated macro for impl_7463 (impl)
macro_rules! Depcrate_mixed_read_write_in_expressionimpl_7463 {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"impl_7463"}
// Dependencies: {}
impl < 'tcx > DivergenceVisitor < '_ , 'tcx > { fn maybe_walk_expr (& mut self , e : & 'tcx Expr < '_ >) { match e . kind { ExprKind :: Closure (..) | ExprKind :: If (..) | ExprKind :: Loop (..) => { } , ExprKind :: Match (e , arms , _) => { self . visit_expr (e) ; for arm in arms { if let Some (if_expr) = arm . guard { self . visit_expr (if_expr) ; } self . maybe_walk_expr (arm . body) ; } } , _ => walk_expr (self , e) , } } fn report_diverging_sub_expr (& self , e : & Expr < '_ >) { if let Some (macro_call) = root_macro_call_first_node (self . cx , e) && self . cx . tcx . is_diagnostic_item (sym :: todo_macro , macro_call . def_id) { return ; } span_lint (self . cx , DIVERGING_SUB_EXPRESSION , e . span , "sub-expression diverges") ; } }
};
}
