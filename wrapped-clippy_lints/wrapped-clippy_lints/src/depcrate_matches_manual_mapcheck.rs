// Generated macro for check (function)
macro_rules! Depcrate_matches_manual_mapcheck {
() => {
// Module: crate::matches::manual_map
// Provides: {"check"}
// Dependencies: {}
fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , scrutinee : & 'tcx Expr < '_ > , then_pat : & 'tcx Pat < '_ > , then_body : & 'tcx Expr < '_ > , else_pat : Option < & 'tcx Pat < '_ > > , else_body : & 'tcx Expr < '_ > ,) { if let Some (sugg_info) = check_with (cx , expr , scrutinee , then_pat , then_body , else_pat , else_body , get_some_expr ,) { span_lint_and_sugg (cx , MANUAL_MAP , expr . span , "manual implementation of `Option::map`" , "try" , if sugg_info . needs_brackets { format ! ("{{ {}{}.map({}) }}" , sugg_info . scrutinee_str , sugg_info . as_ref_str , sugg_info . body_str) } else { format ! ("{}{}.map({})" , sugg_info . scrutinee_str , sugg_info . as_ref_str , sugg_info . body_str) } , sugg_info . app ,) ; } }
};
}
