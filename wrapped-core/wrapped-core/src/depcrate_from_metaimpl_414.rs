// Generated macro for impl_414 (impl)
macro_rules! Depcrate_from_metaimpl_414 {
() => {
// Module: crate::from_meta
// Provides: {"impl_414"}
// Dependencies: {}
impl FromMeta for syn :: Ident { fn from_string (value : & str) -> Result < Self > { syn :: parse_str (value) . map_err (| _ | Error :: unknown_value (value)) } fn from_value (value : & syn :: Lit) -> Result < Self > { if let syn :: Lit :: Str (ref v) = * value { v . parse () . map_err (| _ | Error :: unknown_lit_str_value (v)) } else { Err (Error :: unexpected_lit_type (value)) } } fn from_expr (expr : & Expr) -> Result < Self > { match expr { Expr :: Lit (lit) => Self :: from_value (& lit . lit) , Expr :: Path (path) => match path . path . get_ident () { Some (ident) => Ok (ident . clone ()) , None => Err (Error :: unexpected_expr_type (expr)) , } , Expr :: Group (group) => Self :: from_expr (& group . expr) , _ => Err (Error :: unexpected_expr_type (expr)) , } } }
};
}
