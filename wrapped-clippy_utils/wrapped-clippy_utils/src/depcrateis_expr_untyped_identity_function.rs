// Generated macro for is_expr_untyped_identity_function (function)
macro_rules! Depcrateis_expr_untyped_identity_function {
() => {
// Module: crate
// Provides: {"is_expr_untyped_identity_function"}
// Dependencies: {}
# [doc = " This is the same as [`is_expr_identity_function`], but does not consider closures"] # [doc = " with type annotations for its bindings (or similar) as identity functions:"] # [doc = " * `|x: u8| x`"] # [doc = " * `std::convert::identity::<u8>`"] pub fn is_expr_untyped_identity_function (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: Closure (& Closure { body , fn_decl , .. }) if fn_decl . inputs . iter () . all (| ty | matches ! (ty . kind , TyKind :: Infer (()))) => { is_body_identity_function (cx , cx . tcx . hir_body (body)) } , ExprKind :: Path (QPath :: Resolved (_ , path)) if path . segments . iter () . all (| seg | seg . infer_args) && let Some (did) = path . res . opt_def_id () => { cx . tcx . is_diagnostic_item (sym :: convert_identity , did) } , _ => false , } }
};
}
