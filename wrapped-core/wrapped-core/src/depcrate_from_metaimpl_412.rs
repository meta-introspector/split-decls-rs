// Generated macro for impl_412 (impl)
macro_rules! Depcrate_from_metaimpl_412 {
() => {
// Module: crate::from_meta
// Provides: {"impl_412"}
// Dependencies: {}
# [doc = " Support for arbitrary expressions as values in a meta item."] # [doc = ""] # [doc = " For backwards-compatibility to versions of `darling` based on `syn` 1,"] # [doc = " string literals will be \"unwrapped\" and their contents will be parsed"] # [doc = " as an expression."] # [doc = ""] # [doc = " See [`util::parse_expr`](crate::util::parse_expr) for functions to provide"] # [doc = " alternate parsing modes for this type."] impl FromMeta for syn :: Expr { fn from_expr (expr : & Expr) -> Result < Self > { match expr { Expr :: Lit (syn :: ExprLit { lit : lit @ syn :: Lit :: Str (_) , .. }) => Self :: from_value (lit) , Expr :: Group (group) => Self :: from_expr (& group . expr) , _ => Ok (expr . clone ()) , } } fn from_string (value : & str) -> Result < Self > { syn :: parse_str (value) . map_err (| _ | Error :: unknown_value (value)) } fn from_value (value : & :: syn :: Lit) -> Result < Self > { if let :: syn :: Lit :: Str (ref v) = * value { v . parse :: < syn :: Expr > () . map_err (| _ | Error :: unknown_lit_str_value (v)) } else { Err (Error :: unexpected_lit_type (value)) } } }
};
}
