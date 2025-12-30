// Generated macro for PreservedStrExpr (struct)
macro_rules! Depcrate_util_preserved_str_exprPreservedStrExpr {
() => {
// Module: crate::util::preserved_str_expr
// Provides: {"PreservedStrExpr"}
// Dependencies: {}
# [doc = " A wrapper around [`Expr`] that preserves the original expression"] # [doc = " without evaluating it."] # [doc = ""] # [doc = " For compatibility reasons, `darling` evaluates the expression inside string"] # [doc = " literals, which might be undesirable. In many cases,"] # [doc = " [`darling::util::parse_expr::preserve_str_literal`] can be used. However,"] # [doc = " when using [`Expr`] inside a container (such as a"] # [doc = " [`HashMap`](std::collections::HashMap)), it is not possible to use it."] # [doc = ""] # [doc = " This wrapper preserves the original expression without evaluating it."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[derive(FromMeta)]"] # [doc = " #[darling(attributes(demo))]"] # [doc = " struct Demo {"] # [doc = "     option: Option<HashMap<syn::Ident, PreservedStrExpr>>,"] # [doc = " }"] # [doc = " ```"] # [repr (transparent)] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub struct PreservedStrExpr (pub Expr) ;
};
}
