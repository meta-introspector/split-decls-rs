// Generated macro for impl_7607 (impl)
macro_rules! Depcrate_mixed_read_write_in_expressionimpl_7607 {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"impl_7607"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ReadVisitor < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) { if expr . hir_id == self . last_expr . hir_id { return ; } if expr . res_local_id () == Some (self . var) && ! is_in_assignment_position (self . cx , expr) { span_lint_and_then (self . cx , MIXED_READ_WRITE_IN_EXPRESSION , expr . span , format ! ("unsequenced read of `{}`" , self . cx . tcx . hir_name (self . var)) , | diag | { diag . span_note (self . write_expr . span , "whether read occurs before this write depends on evaluation order" ,) ; } ,) ; } match expr . kind { ExprKind :: Closure { .. } | ExprKind :: AddrOf (_ , _ , _) => { return ; } _ => { } } walk_expr (self , expr) ; } }
};
}
