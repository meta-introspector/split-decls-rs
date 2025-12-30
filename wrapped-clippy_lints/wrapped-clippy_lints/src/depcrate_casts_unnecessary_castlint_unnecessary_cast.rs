// Generated macro for lint_unnecessary_cast (function)
macro_rules! Depcrate_casts_unnecessary_castlint_unnecessary_cast {
() => {
// Module: crate::casts::unnecessary_cast
// Provides: {"lint_unnecessary_cast"}
// Dependencies: {}
fn lint_unnecessary_cast (cx : & LateContext < '_ > , expr : & Expr < '_ > , raw_literal_str : & str , cast_from : Ty < '_ > , cast_to : Ty < '_ > ,) { let literal_kind_name = if cast_from . is_integral () { "integer" } else { "float" } ; let literal_str = raw_literal_str . replace (['(' , ')'] , "") . trim_end_matches ('.') . to_string () ; let sugg = if let Some (parent_expr) = get_parent_expr (cx , expr) && let ExprKind :: MethodCall (..) = parent_expr . kind && literal_str . starts_with ('-') { format ! ("({literal_str}_{cast_to})") } else { format ! ("{literal_str}_{cast_to}") } ; span_lint_and_sugg (cx , UNNECESSARY_CAST , expr . span , format ! ("casting {literal_kind_name} literal to `{cast_to}` is unnecessary") , "try" , sugg , Applicability :: MachineApplicable ,) ; }
};
}
