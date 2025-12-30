// Generated macro for check_if_let (function)
macro_rules! Depcrate_matches_collapsible_matchcheck_if_let {
() => {
// Module: crate::matches::collapsible_match
// Provides: {"check_if_let"}
// Dependencies: {}
pub (super) fn check_if_let < 'tcx > (cx : & LateContext < 'tcx > , pat : & 'tcx Pat < '_ > , body : & 'tcx Expr < '_ > , else_expr : Option < & 'tcx Expr < '_ > > , let_expr : & 'tcx Expr < '_ > , msrv : Msrv ,) { check_arm (cx , false , pat , let_expr , body , None , else_expr , msrv) ; }
};
}
