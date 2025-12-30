// Generated macro for impl_9606 (impl)
macro_rules! Depcrate_slow_vector_initializationimpl_9606 {
() => {
// Module: crate::slow_vector_initialization
// Provides: {"impl_9606"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SlowVectorInit { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Assign (left , right , _) = expr . kind && let Some (local_id) = path_to_local (left) && let Some (size_expr) = Self :: as_vec_initializer (cx , right) { let vi = VecAllocation { local_id , allocation_expr : right , size_expr , } ; Self :: search_initialization (cx , vi , expr . hir_id) ; } } fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ >) { if let StmtKind :: Let (local) = stmt . kind && let PatKind :: Binding (BindingMode :: MUT , local_id , _ , None) = local . pat . kind && let Some (init) = local . init && let Some (size_expr) = Self :: as_vec_initializer (cx , init) { let vi = VecAllocation { local_id , allocation_expr : init , size_expr , } ; Self :: search_initialization (cx , vi , stmt . hir_id) ; } } }
};
}
