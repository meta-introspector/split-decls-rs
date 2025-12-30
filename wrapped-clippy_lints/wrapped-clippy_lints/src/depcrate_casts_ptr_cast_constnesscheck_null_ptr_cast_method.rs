// Generated macro for check_null_ptr_cast_method (function)
macro_rules! Depcrate_casts_ptr_cast_constnesscheck_null_ptr_cast_method {
() => {
// Module: crate::casts::ptr_cast_constness
// Provides: {"check_null_ptr_cast_method"}
// Dependencies: {}
pub (super) fn check_null_ptr_cast_method (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: MethodCall (method , cast_from_expr , [] , _) = expr . kind && let ExprKind :: Call (func , []) = cast_from_expr . kind && let ExprKind :: Path (QPath :: Resolved (None , path)) = func . kind && let Some (defid) = path . res . opt_def_id () && let method = match (cx . tcx . get_diagnostic_name (defid) , method . ident . name) { (Some (sym :: ptr_null) , sym :: cast_mut) => "null_mut" , (Some (sym :: ptr_null_mut) , sym :: cast_const) => "null" , _ => return , } && let Some (prefix) = std_or_core (cx) && let mut app = Applicability :: MachineApplicable && let sugg = snippet_with_applicability (cx , cast_from_expr . span , "_" , & mut app) && let Some ((_ , after_lt)) = sugg . split_once ("::<") { span_lint_and_sugg (cx , PTR_CAST_CONSTNESS , expr . span , "changing constness of a null pointer" , format ! ("use `{method}()` directly instead") , format ! ("{prefix}::ptr::{method}::<{after_lt}") , app ,) ; } }
};
}
