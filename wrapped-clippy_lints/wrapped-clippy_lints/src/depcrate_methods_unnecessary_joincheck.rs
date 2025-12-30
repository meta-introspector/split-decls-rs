// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_joincheck {
() => {
// Module: crate::methods::unnecessary_join
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , join_self_arg : & 'tcx Expr < 'tcx > , join_arg : & 'tcx Expr < 'tcx > , span : Span ,) { let applicability = Applicability :: MachineApplicable ; let collect_output_adjusted_type = cx . typeck_results () . expr_ty_adjusted (join_self_arg) ; if let ty :: Ref (_ , ref_type , _) = collect_output_adjusted_type . kind () && let ty :: Slice (slice) = * ref_type . kind () && slice . is_lang_item (cx , LangItem :: String) && let ExprKind :: Lit (spanned) = & join_arg . kind && let LitKind :: Str (symbol , _) = spanned . node && symbol . is_empty () { span_lint_and_sugg (cx , UNNECESSARY_JOIN , span . with_hi (expr . span . hi ()) , r#"called `.collect::<Vec<String>>().join("")` on an iterator"# , "consider using" , "collect::<String>()" . to_owned () , applicability ,) ; } }
};
}
