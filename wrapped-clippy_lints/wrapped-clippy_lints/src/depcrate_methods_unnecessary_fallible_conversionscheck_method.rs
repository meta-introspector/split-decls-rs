// Generated macro for check_method (function)
macro_rules! Depcrate_methods_unnecessary_fallible_conversionscheck_method {
() => {
// Module: crate::methods::unnecessary_fallible_conversions
// Provides: {"check_method"}
// Dependencies: {}
# [doc = " Checks method call exprs:"] # [doc = " - `0i32.try_into()`"] pub (super) fn check_method (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: MethodCall (path , ..) = expr . kind { check (cx , expr , cx . typeck_results () . node_args (expr . hir_id) , FunctionKind :: TryIntoMethod , path . ident . span ,) ; } }
};
}
