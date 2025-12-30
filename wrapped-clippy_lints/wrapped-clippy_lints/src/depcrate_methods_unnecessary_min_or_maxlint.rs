// Generated macro for lint (function)
macro_rules! Depcrate_methods_unnecessary_min_or_maxlint {
() => {
// Module: crate::methods::unnecessary_min_or_max
// Provides: {"lint"}
// Dependencies: {}
fn lint (cx : & LateContext < '_ > , expr : & Expr < '_ > , name : Symbol , lhs : Span , rhs : Span , order : Ordering) { let cmp_str = if order . is_ge () { "smaller" } else { "greater" } ; let suggested_value = if (name == sym :: min && order . is_ge ()) || (name == sym :: max && order . is_le ()) { snippet (cx , rhs , "..") } else { snippet (cx , lhs , "..") } ; span_lint_and_sugg (cx , UNNECESSARY_MIN_OR_MAX , expr . span , format ! ("`{}` is never {} than `{}` and has therefore no effect" , snippet (cx , lhs , "..") , cmp_str , snippet (cx , rhs , "..")) , "try" , suggested_value . to_string () , Applicability :: MachineApplicable ,) ; }
};
}
