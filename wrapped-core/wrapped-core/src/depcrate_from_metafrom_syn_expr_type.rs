// Generated macro for from_syn_expr_type (macro)
macro_rules! Depcrate_from_metafrom_syn_expr_type {
() => {
// Module: crate::from_meta
// Provides: {"from_syn_expr_type"}
// Dependencies: {}
# [doc = " Adapter for various expression types."] # [doc = ""] # [doc = " Prior to syn 2.0, darling supported arbitrary expressions as long as they"] # [doc = " were wrapped in quotation marks. This was helpful for people writing"] # [doc = " libraries that needed expressions, but it now creates an ambiguity when"] # [doc = " parsing a meta item."] # [doc = ""] # [doc = " To address this, the macro supports both formats; if it cannot parse the"] # [doc = " item as an expression of the right type and the passed-in expression is"] # [doc = " a string literal, it will fall back to parsing the string contents."] macro_rules ! from_syn_expr_type { ($ ty : path , $ variant : ident) => { impl FromMeta for $ ty { fn from_expr (expr : & syn :: Expr) -> Result < Self > { match expr { syn :: Expr ::$ variant (body) => Ok (body . clone ()) , syn :: Expr :: Lit (expr_lit) => Self :: from_value (& expr_lit . lit) , syn :: Expr :: Group (group) => Self :: from_expr (& group . expr) , _ => Err (Error :: unexpected_expr_type (expr)) , } } fn from_value (value : &:: syn :: Lit) -> Result < Self > { if let syn :: Lit :: Str (body) = & value { body . parse ::<$ ty > () . map_err (| _ | Error :: unknown_lit_str_value (body)) } else { Err (Error :: unexpected_lit_type (value)) } } } } ; }
};
}
