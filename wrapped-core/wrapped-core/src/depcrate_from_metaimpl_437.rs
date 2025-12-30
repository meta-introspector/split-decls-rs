// Generated macro for impl_437 (impl)
macro_rules! Depcrate_from_metaimpl_437 {
() => {
// Module: crate::from_meta
// Provides: {"impl_437"}
// Dependencies: {}
impl FromMeta for syn :: TypePath { # [doc = " Supports both quote-wrapped and bare values."] fn from_expr (expr : & Expr) -> Result < Self > { match expr { Expr :: Path (body) => { if body . attrs . is_empty () { Ok (syn :: TypePath { qself : body . qself . clone () , path : body . path . clone () , }) } else { Err (Error :: custom ("attributes are not allowed") . with_span (body)) } } Expr :: Lit (expr_lit) => Self :: from_value (& expr_lit . lit) , Expr :: Group (group) => Self :: from_expr (& group . expr) , _ => Err (Error :: unexpected_expr_type (expr)) , } } fn from_string (value : & str) -> Result < Self > { syn :: parse_str (value) . map_err (| _ | Error :: unknown_value (value)) } fn from_value (value : & Lit) -> Result < Self > { if let Lit :: Str (ref v) = * value { v . parse () . map_err (| _ | Error :: unknown_lit_str_value (v)) } else { Err (Error :: unexpected_lit_type (value)) } } }
};
}
