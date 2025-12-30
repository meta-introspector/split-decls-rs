// Generated macro for impl_6054 (impl)
macro_rules! Depcrate_methods_needless_collectimpl_6054 {
() => {
// Module: crate::methods::needless_collect
// Provides: {"impl_6054"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for IteratorMethodCheckVisitor < '_ , 'tcx > { type Result = ControlFlow < () > ; fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> ControlFlow < () > { if let ExprKind :: MethodCall (_method_name , recv , _args , _) = & expr . kind && (recv . hir_id == self . hir_id_of_expr || self . hir_id_of_let_binding . is_some_and (| hid | path_to_local_id (recv , hid))) && ! is_trait_method (self . cx , expr , sym :: Iterator) { return ControlFlow :: Break (()) ; } else if let ExprKind :: Assign (place , value , _span) = & expr . kind && value . hir_id == self . hir_id_of_expr && let Some (id) = path_to_local (place) { self . hir_id_of_let_binding = Some (id) ; } walk_expr (self , expr) } fn visit_stmt (& mut self , stmt : & 'tcx Stmt < 'tcx >) -> ControlFlow < () > { if let StmtKind :: Let (LetStmt { init : Some (expr) , pat : Pat { kind : PatKind :: Binding (BindingMode :: NONE | BindingMode :: MUT , id , _ , None) , .. } , .. }) = & stmt . kind && expr . hir_id == self . hir_id_of_expr { self . hir_id_of_let_binding = Some (* id) ; } walk_stmt (self , stmt) } }
};
}
