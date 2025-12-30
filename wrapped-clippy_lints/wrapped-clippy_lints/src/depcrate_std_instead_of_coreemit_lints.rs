// Generated macro for emit_lints (function)
macro_rules! Depcrate_std_instead_of_coreemit_lints {
() => {
// Module: crate::std_instead_of_core
// Provides: {"emit_lints"}
// Dependencies: {}
fn emit_lints (cx : & LateContext < '_ > , lint_points : Option < (Span , Vec < LintPoint >) >) { let Some ((krate_span , lint_points)) = lint_points else { return ; } ; let mut lint : Option < (& 'static Lint , & 'static str , & 'static str) > = None ; let mut has_conflict = false ; for lint_point in & lint_points { match lint_point { LintPoint :: Available (_ , l , used_mod , replace_with) if lint . is_none_or (| (prev_l , ..) | l . name == prev_l . name) => { lint = Some ((l , used_mod , replace_with)) ; } , _ => { has_conflict = true ; break ; } , } } if ! has_conflict && let Some ((lint , used_mod , replace_with)) = lint { span_lint_and_sugg (cx , lint , krate_span , format ! ("used import from `{used_mod}` instead of `{replace_with}`") , format ! ("consider importing the item from `{replace_with}`") , (* replace_with) . to_string () , Applicability :: MachineApplicable ,) ; return ; } for lint_point in lint_points { let LintPoint :: Available (span , lint , used_mod , replace_with) = lint_point else { continue ; } ; span_lint_and_help (cx , lint , span , format ! ("used import from `{used_mod}` instead of `{replace_with}`") , None , format ! ("consider importing the item from `{replace_with}`") ,) ; } }
};
}
