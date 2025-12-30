// Generated macro for is_expr_identity_function (function)
macro_rules! Depcrateis_expr_identity_function {
() => {
// Module: crate
// Provides: {"is_expr_identity_function"}
// Dependencies: {}
# [doc = " Checks if an expression represents the identity function"] # [doc = " Only examines closures and `std::convert::identity`"] # [doc = ""] # [doc = " NOTE: If you want to use this function to find out if a closure is unnecessary, you likely want"] # [doc = " to call [`is_expr_untyped_identity_function`] instead, which makes sure that the closure doesn't"] # [doc = " have type annotations. This is important because removing a closure with bindings can"] # [doc = " remove type information that helped type inference before, which can then lead to compile"] # [doc = " errors."] pub fn is_expr_identity_function (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: Closure (& Closure { body , .. }) => is_body_identity_function (cx , cx . tcx . hir_body (body)) , _ => expr . basic_res () . is_diag_item (cx , sym :: convert_identity) , } }
};
}
