// Generated macro for check (function)
macro_rules! Depcrate_matches_manual_filtercheck {
() => {
// Module: crate::matches::manual_filter
// Provides: {"check"}
// Dependencies: {}
fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , scrutinee : & 'tcx Expr < '_ > , then_pat : & 'tcx Pat < '_ > , then_body : & 'tcx Expr < '_ > , else_pat : Option < & 'tcx Pat < '_ > > , else_body : & 'tcx Expr < '_ > ,) { if let Some (sugg_info) = check_with (cx , expr , scrutinee , then_pat , then_body , else_pat , else_body , get_cond_expr ,) { let body_str = add_ampersand_if_copy (sugg_info . body_str , sugg_info . scrutinee_impl_copy) ; span_lint_and_sugg (cx , MANUAL_FILTER , expr . span , "manual implementation of `Option::filter`" , "try" , if sugg_info . needs_brackets { format ! ("{{ {}{}.filter({body_str}) }}" , sugg_info . scrutinee_str , sugg_info . as_ref_str) } else { format ! ("{}{}.filter({body_str})" , sugg_info . scrutinee_str , sugg_info . as_ref_str) } , sugg_info . app ,) ; } }
};
}
