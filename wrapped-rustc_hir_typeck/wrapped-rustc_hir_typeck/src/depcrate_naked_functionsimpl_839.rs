// Generated macro for impl_839 (impl)
macro_rules! Depcrate_naked_functionsimpl_839 {
() => {
// Module: crate::naked_functions
// Provides: {"impl_839"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for CheckInlineAssembly { fn visit_stmt (& mut self , stmt : & 'tcx hir :: Stmt < 'tcx >) { match stmt . kind { StmtKind :: Item (..) => { } StmtKind :: Let (..) => { self . items . push ((ItemKind :: NonAsm , stmt . span)) ; } StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => { self . check_expr (expr , stmt . span) ; } } } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { self . check_expr (expr , expr . span) ; } }
};
}
