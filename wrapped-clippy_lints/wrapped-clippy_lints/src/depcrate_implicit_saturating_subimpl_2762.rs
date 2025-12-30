// Generated macro for impl_2762 (impl)
macro_rules! Depcrate_implicit_saturating_subimpl_2762 {
() => {
// Module: crate::implicit_saturating_sub
// Provides: {"impl_2762"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ImplicitSaturatingSub { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if expr . span . from_expansion () { return ; } if let Some (higher :: If { cond , then , r#else : None }) = higher :: If :: hir (expr) && let ExprKind :: Binary (ref cond_op , cond_left , cond_right) = cond . kind { check_with_condition (cx , expr , cond_op . node , cond_left , cond_right , then) ; } else if let Some (higher :: If { cond , then : if_block , r#else : Some (else_block) , }) = higher :: If :: hir (expr) && let ExprKind :: Binary (ref cond_op , cond_left , cond_right) = cond . kind { check_manual_check (cx , expr , cond_op , cond_left , cond_right , if_block , else_block , self . msrv ,) ; } } }
};
}
