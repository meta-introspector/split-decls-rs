// Generated macro for impl_2034 (impl)
macro_rules! Depcrate_eta_reductionimpl_2034 {
() => {
// Module: crate::eta_reduction
// Provides: {"impl_2034"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for EtaReduction { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if let ExprKind :: MethodCall (_method , receiver , args , _) = expr . kind { for arg in args { check_closure (cx , Some (receiver) , arg) ; } } if let ExprKind :: Call (func , args) = expr . kind { check_closure (cx , None , func) ; for arg in args { check_closure (cx , None , arg) ; } } } }
};
}
