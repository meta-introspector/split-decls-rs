// Generated macro for impl_7601 (impl)
macro_rules! Depcrate_mixed_read_write_in_expressionimpl_7601 {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"impl_7601"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for DivergenceVisitor < '_ , 'tcx > { fn visit_expr (& mut self , e : & 'tcx Expr < '_ >) { match e . kind { ExprKind :: Block (block , ..) => match (block . stmts , block . expr) { (stmts , Some (e)) => { if stmts . iter () . all (| stmt | ! stmt_might_diverge (stmt)) { self . visit_expr (e) ; } } , ([first @ .. , stmt] , None) => { if first . iter () . all (| stmt | ! stmt_might_diverge (stmt)) { match stmt . kind { StmtKind :: Expr (e) | StmtKind :: Semi (e) => self . visit_expr (e) , _ => { } , } } } , _ => { } , } , ExprKind :: Continue (_) | ExprKind :: Break (_ , _) | ExprKind :: Ret (_) => self . report_diverging_sub_expr (e) , ExprKind :: Call (func , _) => { let typ = self . cx . typeck_results () . expr_ty (func) ; if typ . is_fn () { let sig = typ . fn_sig (self . cx . tcx) ; if self . cx . tcx . instantiate_bound_regions_with_erased (sig) . output () . kind () == & ty :: Never { self . report_diverging_sub_expr (e) ; } } } , ExprKind :: MethodCall (..) => { let borrowed_table = self . cx . typeck_results () ; if borrowed_table . expr_ty (e) . is_never () { self . report_diverging_sub_expr (e) ; } } , _ => { } , } self . maybe_walk_expr (e) ; } fn visit_block (& mut self , _ : & 'tcx Block < '_ >) { } }
};
}
