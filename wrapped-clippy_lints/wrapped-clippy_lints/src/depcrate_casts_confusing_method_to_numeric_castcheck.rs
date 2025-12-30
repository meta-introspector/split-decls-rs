// Generated macro for check (function)
macro_rules! Depcrate_casts_confusing_method_to_numeric_castcheck {
() => {
// Module: crate::casts::confusing_method_to_numeric_cast
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , cast_expr : & Expr < '_ > , cast_from : Ty < '_ > , cast_to : Ty < '_ >) { if cast_to . is_fn () { return ; } if let ty :: FnDef (def_id , generics) = cast_from . kind () && let Some (method_name) = cx . tcx . opt_item_name (* def_id) && let Some ((const_name , ty_name)) = get_const_name_and_ty_name (cx , method_name , * def_id , generics . as_slice ()) { let mut applicability = Applicability :: MaybeIncorrect ; let from_snippet = snippet_with_applicability (cx , cast_expr . span , ".." , & mut applicability) ; span_lint_and_then (cx , CONFUSING_METHOD_TO_NUMERIC_CAST , expr . span , format ! ("casting function pointer `{from_snippet}` to `{cast_to}`") , | diag | { diag . span_suggestion_verbose (expr . span , "did you mean to use the associated constant?" , format ! ("{ty_name}::{const_name} as {cast_to}") , applicability ,) ; } ,) ; } }
};
}
