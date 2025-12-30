// Generated macro for impl_9610 (impl)
macro_rules! Depcrate_slow_vector_initializationimpl_9610 {
() => {
// Module: crate::slow_vector_initialization
// Provides: {"impl_9610"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for VectorInitializationVisitor < '_ , 'tcx > { fn visit_stmt (& mut self , stmt : & 'tcx Stmt < '_ >) { if self . initialization_found { match stmt . kind { StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => { self . search_slow_extend_filling (expr) ; self . search_slow_resize_filling (expr) ; } , _ => () , } self . initialization_found = false ; } else { walk_stmt (self , stmt) ; } } fn visit_block (& mut self , block : & 'tcx Block < '_ >) { if self . initialization_found { if let Some (s) = block . stmts . first () { self . visit_stmt (s) ; } self . initialization_found = false ; } else { walk_block (self , block) ; } } fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) { if self . vec_alloc . allocation_expr . hir_id == expr . hir_id { self . initialization_found = true ; } walk_expr (self , expr) ; } }
};
}
