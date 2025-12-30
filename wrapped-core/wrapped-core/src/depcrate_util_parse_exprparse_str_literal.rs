// Generated macro for parse_str_literal (function)
macro_rules! Depcrate_util_parse_exprparse_str_literal {
() => {
// Module: crate::util::parse_expr
// Provides: {"parse_str_literal"}
// Dependencies: {}
# [doc = " Parse a [`Meta`] to an [`Expr`]; if the value is a string literal, the string's"] # [doc = " contents will be parsed as an expression and emitted."] pub fn parse_str_literal (meta : & Meta) -> crate :: Result < Expr > { match meta { Meta :: Path (_) => Err (Error :: unsupported_format ("path") . with_span (meta)) , Meta :: List (_) => Err (Error :: unsupported_format ("list") . with_span (meta)) , Meta :: NameValue (nv) => { if let Expr :: Lit (expr_lit) = & nv . value { Expr :: from_value (& expr_lit . lit) } else { Ok (nv . value . clone ()) } } } }
};
}
