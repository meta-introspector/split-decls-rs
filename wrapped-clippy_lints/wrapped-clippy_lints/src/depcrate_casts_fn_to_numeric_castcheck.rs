// Generated macro for check (function)
macro_rules! Depcrate_casts_fn_to_numeric_castcheck {
() => {
// Module: crate::casts::fn_to_numeric_cast
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , cast_expr : & Expr < '_ > , cast_from : Ty < '_ > , cast_to : Ty < '_ >) { let Some (to_nbits) = utils :: int_ty_to_nbits (cx . tcx , cast_to) else { return ; } ; if cast_from . is_fn () { let mut applicability = Applicability :: MaybeIncorrect ; if to_nbits >= cx . tcx . data_layout . pointer_size () . bits () && ! cast_to . is_usize () { let from_snippet = snippet_with_applicability (cx , cast_expr . span , "x" , & mut applicability) ; span_lint_and_sugg (cx , FN_TO_NUMERIC_CAST , expr . span , format ! ("casting function pointer `{from_snippet}` to `{cast_to}`") , "try" , format ! ("{from_snippet} as usize") , applicability ,) ; } } }
};
}
