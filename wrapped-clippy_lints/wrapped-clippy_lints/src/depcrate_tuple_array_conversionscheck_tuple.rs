// Generated macro for check_tuple (function)
macro_rules! Depcrate_tuple_array_conversionscheck_tuple {
() => {
// Module: crate::tuple_array_conversions
// Provides: {"check_tuple"}
// Dependencies: {}
fn check_tuple < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , elements : & 'tcx [Expr < 'tcx >]) { if let ty :: Tuple (tys) = cx . typeck_results () . expr_ty (expr) . kind () && let [first , ..] = elements && tys . iter () . all_equal () && let Some (locals) = (match first . kind { ExprKind :: Index (..) => elements . iter () . enumerate () . map (| (i , i_expr) | -> Option < & 'tcx Expr < 'tcx > > { if let ExprKind :: Index (lhs , index , _) = i_expr . kind && let ExprKind :: Lit (lit) = index . kind && let LitKind :: Int (val , _) = lit . node { return (val == i as u128) . then_some (lhs) ; } None }) . collect :: < Option < Vec < _ > > > () , ExprKind :: Path (_) => Some (elements . iter () . collect ()) , _ => None , }) && all_bindings_are_for_conv (cx , tys , expr , elements , & locals , ToType :: Tuple) && ! is_from_proc_macro (cx , expr) { span_lint_and_help (cx , TUPLE_ARRAY_CONVERSIONS , expr . span , "it looks like you're trying to convert an array to a tuple" , None , "use `.into()` instead, or `<(T0, T1, ..., Tn)>::from` if type annotations are needed" ,) ; } }
};
}
