// Generated macro for check (function)
macro_rules! Depcrate_casts_cast_sign_losscheck {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'cx > (cx : & LateContext < 'cx > , expr : & Expr < '_ > , cast_op : & Expr < '_ > , cast_from : Ty < 'cx > , cast_to : Ty < '_ > , msrv : Msrv ,) { if should_lint (cx , cast_op , cast_from , cast_to) { span_lint_and_then (cx , CAST_SIGN_LOSS , expr . span , format ! ("casting `{cast_from}` to `{cast_to}` may lose the sign of the value") , | diag | { if msrv . meets (cx , msrvs :: INTEGER_SIGN_CAST) && let Some (cast) = utils :: is_signedness_cast (cast_from , cast_to) { let method = match cast { utils :: CastTo :: Signed => "cast_signed()" , utils :: CastTo :: Unsigned => "cast_unsigned()" , } ; let mut app = Applicability :: MaybeIncorrect ; let sugg = Sugg :: hir_with_context (cx , cast_op , expr . span . ctxt () , ".." , & mut app) ; diag . span_suggestion (expr . span , format ! ("if this is intentional, use `{method}` instead") , format ! ("{}.{method}" , sugg . maybe_paren ()) , app ,) ; } } ,) ; } }
};
}
