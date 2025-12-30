// Generated macro for impl_10401 (impl)
macro_rules! Depcrate_uninit_vecimpl_10401 {
() => {
// Module: crate::uninit_vec
// Provides: {"impl_10401"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UninitVec { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx Block < '_ >) { if ! block . span . in_external_macro (cx . tcx . sess . source_map ()) { for w in block . stmts . windows (2) { if let StmtKind :: Expr (expr) | StmtKind :: Semi (expr) = w [1] . kind { handle_uninit_vec_pair (cx , & w [0] , expr) ; } } if let (Some (stmt) , Some (expr)) = (block . stmts . last () , block . expr) { handle_uninit_vec_pair (cx , stmt , expr) ; } } } }
};
}
