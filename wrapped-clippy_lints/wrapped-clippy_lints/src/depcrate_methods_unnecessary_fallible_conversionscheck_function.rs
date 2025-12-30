// Generated macro for check_function (function)
macro_rules! Depcrate_methods_unnecessary_fallible_conversionscheck_function {
() => {
// Module: crate::methods::unnecessary_fallible_conversions
// Provides: {"check_function"}
// Dependencies: {}
# [doc = " Checks function call exprs:"] # [doc = " - `<i64 as TryFrom<_>>::try_from(0i32)`"] # [doc = " - `<_ as TryInto<i64>>::try_into(0i32)`"] pub (super) fn check_function (cx : & LateContext < '_ > , expr : & Expr < '_ > , callee : & Expr < '_ >) { if let ExprKind :: Path (ref qpath) = callee . kind && let Some (item_def_id) = cx . qpath_res (qpath , callee . hir_id) . opt_def_id () && let Some (trait_def_id) = cx . tcx . trait_of_assoc (item_def_id) { let qpath_spans = match qpath { QPath :: Resolved (_ , path) => { if let [trait_seg , fn_seg] = path . segments { Some (SpansKind :: TraitFn { trait_span : trait_seg . ident . span , fn_span : fn_seg . ident . span , }) } else { None } } , QPath :: TypeRelative (_ , seg) => Some (SpansKind :: Fn { fn_span : seg . ident . span , }) , } ; check (cx , expr , cx . typeck_results () . node_args (callee . hir_id) , match cx . tcx . get_diagnostic_name (trait_def_id) { Some (sym :: TryFrom) => FunctionKind :: TryFromFunction (qpath_spans) , Some (sym :: TryInto) => FunctionKind :: TryIntoFunction (qpath_spans) , _ => return , } , callee . span ,) ; } }
};
}
