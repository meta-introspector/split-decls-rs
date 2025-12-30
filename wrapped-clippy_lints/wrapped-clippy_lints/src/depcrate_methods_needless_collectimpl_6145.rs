// Generated macro for impl_6145 (impl)
macro_rules! Depcrate_methods_needless_collectimpl_6145 {
() => {
// Module: crate::methods::needless_collect
// Provides: {"impl_6145"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for IteratorMethodCheckVisitor < '_ , 'tcx > { type Result = ControlFlow < () > ; fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> ControlFlow < () > { if let ExprKind :: MethodCall (_method_name , recv , _args , _) = & expr . kind && (recv . hir_id == self . hir_id_of_expr || self . hir_id_of_let_binding . is_some_and (| hid | recv . res_local_id () == Some (hid))) && ! self . cx . ty_based_def (expr) . opt_parent (self . cx) . is_diag_item (self . cx , sym :: Iterator) { return ControlFlow :: Break (()) ; } else if let ExprKind :: Assign (place , value , _span) = & expr . kind && value . hir_id == self . hir_id_of_expr && let Some (id) = place . res_local_id () { self . hir_id_of_let_binding = Some (id) ; } walk_expr (self , expr) } fn visit_stmt (& mut self , stmt : & 'tcx Stmt < 'tcx >) -> ControlFlow < () > { if let StmtKind :: Let (LetStmt { init : Some (expr) , pat : Pat { kind : PatKind :: Binding (BindingMode :: NONE | BindingMode :: MUT , id , _ , None) , .. } , .. }) = & stmt . kind && expr . hir_id == self . hir_id_of_expr { self . hir_id_of_let_binding = Some (* id) ; } walk_stmt (self , stmt) } }
};
}
