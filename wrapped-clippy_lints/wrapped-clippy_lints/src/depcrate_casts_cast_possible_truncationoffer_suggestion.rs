// Generated macro for offer_suggestion (function)
macro_rules! Depcrate_casts_cast_possible_truncationoffer_suggestion {
() => {
// Module: crate::casts::cast_possible_truncation
// Provides: {"offer_suggestion"}
// Dependencies: {}
fn offer_suggestion (cx : & LateContext < '_ > , expr : & Expr < '_ > , cast_expr : & Expr < '_ > , cast_to_span : Span , diag : & mut Diag < '_ , () > ,) { let cast_to_snip = snippet (cx , cast_to_span , "..") ; let suggestion = if cast_to_snip == "_" { format ! ("{}.try_into()" , Sugg :: hir (cx , cast_expr , "..") . maybe_paren ()) } else { format ! ("{cast_to_snip}::try_from({})" , Sugg :: hir (cx , cast_expr , "..")) } ; diag . span_suggestion_verbose (expr . span , "... or use `try_from` and handle the error accordingly" , suggestion , Applicability :: Unspecified ,) ; }
};
}
