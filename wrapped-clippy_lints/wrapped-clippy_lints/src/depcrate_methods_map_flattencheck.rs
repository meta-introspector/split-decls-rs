// Generated macro for check (function)
macro_rules! Depcrate_methods_map_flattencheck {
() => {
// Module: crate::methods::map_flatten
// Provides: {"check"}
// Dependencies: {}
# [doc = " lint use of `map().flatten()` for `Iterators` and 'Options'"] pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , map_arg : & Expr < '_ > , map_span : Span) { if let Some ((caller_ty_name , method_to_use)) = try_get_caller_ty_name_and_method_name (cx , expr , recv , map_arg) { let mut applicability = Applicability :: MachineApplicable ; let closure_snippet = snippet_with_applicability (cx , map_arg . span , ".." , & mut applicability) ; let span = expr . span . with_lo (map_span . lo ()) ; if span_contains_comment (cx . tcx . sess . source_map () , span) { applicability = Applicability :: Unspecified ; } span_lint_and_sugg (cx , MAP_FLATTEN , span , format ! ("called `map(..).flatten()` on `{caller_ty_name}`") , format ! ("try replacing `map` with `{method_to_use}` and remove the `.flatten()`") , format ! ("{method_to_use}({closure_snippet})") , applicability ,) ; } }
};
}
