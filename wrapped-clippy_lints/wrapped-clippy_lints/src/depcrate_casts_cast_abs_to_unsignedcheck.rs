// Generated macro for check (function)
macro_rules! Depcrate_casts_cast_abs_to_unsignedcheck {
() => {
// Module: crate::casts::cast_abs_to_unsigned
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , cast_expr : & Expr < '_ > , cast_from : Ty < '_ > , cast_to : Ty < '_ > , msrv : Msrv ,) { if let ty :: Int (from) = cast_from . kind () && let ty :: Uint (to) = cast_to . kind () && let ExprKind :: MethodCall (method_path , receiver , [] , _) = cast_expr . kind && method_path . ident . name == sym :: abs && msrv . meets (cx , msrvs :: UNSIGNED_ABS) { let span = if from . bit_width () == to . bit_width () { expr . span } else { cast_expr . span } ; span_lint_and_sugg (cx , CAST_ABS_TO_UNSIGNED , span , format ! ("casting the result of `{cast_from}::abs()` to {cast_to}") , "replace with" , format ! ("{}.unsigned_abs()" , Sugg :: hir (cx , receiver , "..") . maybe_paren ()) , Applicability :: MachineApplicable ,) ; } }
};
}
