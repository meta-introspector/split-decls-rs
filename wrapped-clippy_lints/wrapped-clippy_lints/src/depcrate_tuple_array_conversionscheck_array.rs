// Generated macro for check_array (function)
macro_rules! Depcrate_tuple_array_conversionscheck_array {
() => {
// Module: crate::tuple_array_conversions
// Provides: {"check_array"}
// Dependencies: {}
fn check_array < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , elements : & 'tcx [Expr < 'tcx >]) { let Some (ty) = cx . typeck_results () . expr_ty (expr) . builtin_index () else { unreachable ! ("`expr` must be an array or slice due to `ExprKind::Array`") ; } ; if let [first , ..] = elements && let Some (locals) = (match first . kind { ExprKind :: Field (_ , _) => elements . iter () . enumerate () . map (| (i , f) | -> Option < & 'tcx Expr < 'tcx > > { let ExprKind :: Field (lhs , ident) = f . kind else { return None ; } ; (ident . name . as_str () == i . to_string ()) . then_some (lhs) }) . collect :: < Option < Vec < _ > > > () , ExprKind :: Path (_) => Some (elements . iter () . collect ()) , _ => None , }) && all_bindings_are_for_conv (cx , & [ty] , expr , elements , & locals , ToType :: Array) && ! is_from_proc_macro (cx , expr) { span_lint_and_help (cx , TUPLE_ARRAY_CONVERSIONS , expr . span , "it looks like you're trying to convert a tuple to an array" , None , "use `.into()` instead, or `<[T; N]>::from` if type annotations are needed" ,) ; } }
};
}
