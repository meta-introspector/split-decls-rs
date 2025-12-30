// Generated macro for impl_235 (impl)
macro_rules! Depcrate_danglingimpl_235 {
() => {
// Module: crate::dangling
// Provides: {"impl_235"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for DanglingPointerReturnSearcher < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> Self :: Result { if let ExprKind :: Ret (Some (expr)) = expr . kind { lint_addr_of_local (self . cx , self . dcx , expr) ; } walk_expr (self , expr) } }
};
}
