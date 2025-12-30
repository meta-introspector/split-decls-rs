// Generated macro for check_if_let (function)
macro_rules! Depcrate_matches_manual_unwrap_orcheck_if_let {
() => {
// Module: crate::matches::manual_unwrap_or
// Provides: {"check_if_let"}
// Dependencies: {}
pub fn check_if_let < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , pat : & 'tcx Pat < 'tcx > , scrutinee : & 'tcx Expr < 'tcx > , then_expr : & 'tcx Expr < 'tcx > , else_expr : & 'tcx Expr < 'tcx > ,) { if let Some (binding_id) = get_some (cx , pat) { handle (cx , expr , "if let" , scrutinee , peel_blocks (then_expr) , peel_blocks (else_expr) , binding_id ,) ; } }
};
}
