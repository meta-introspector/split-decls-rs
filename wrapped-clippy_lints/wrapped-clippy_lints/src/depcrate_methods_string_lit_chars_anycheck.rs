// Generated macro for check (function)
macro_rules! Depcrate_methods_string_lit_chars_anycheck {
() => {
// Module: crate::methods::string_lit_chars_any
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , recv : & Expr < '_ > , param : & 'tcx Param < 'tcx > , body : & Expr < '_ > , msrv : Msrv ,) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let PatKind :: Binding (_ , arg , _ , _) = param . pat . kind && let ExprKind :: Lit (lit_kind) = recv . kind && let LitKind :: Str (val , _) = lit_kind . node && let ExprKind :: Binary (kind , lhs , rhs) = body . kind && let BinOpKind :: Eq = kind . node && let Some (lhs_path) = lhs . res_local_id () && let Some (rhs_path) = rhs . res_local_id () && let scrutinee = match (lhs_path == arg , rhs_path == arg) { (true , false) => rhs , (false , true) => lhs , _ => return , } && msrv . meets (cx , msrvs :: MATCHES_MACRO) && ! is_from_proc_macro (cx , expr) && let Some (scrutinee_snip) = scrutinee . span . get_source_text (cx) { let pat_snip = val . as_str () . chars () . map (| c | format ! ("{c:?}")) . join (" | ") ; span_lint_and_then (cx , STRING_LIT_CHARS_ANY , expr . span , "usage of `.chars().any(...)` to check if a char matches any from a string literal" , | diag | { diag . span_suggestion_verbose (expr . span , "use `matches!(...)` instead" , format ! ("matches!({scrutinee_snip}, {pat_snip})") , Applicability :: MachineApplicable ,) ; } ,) ; } }
};
}
