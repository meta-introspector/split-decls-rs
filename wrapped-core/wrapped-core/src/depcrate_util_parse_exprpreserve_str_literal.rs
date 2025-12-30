// Generated macro for preserve_str_literal (function)
macro_rules! Depcrate_util_parse_exprpreserve_str_literal {
() => {
// Module: crate::util::parse_expr
// Provides: {"preserve_str_literal"}
// Dependencies: {}
# [doc = " Parse a [`Meta`] to an [`Expr`]; if the value is a string literal, the emitted"] # [doc = " expression will be a string literal."] pub fn preserve_str_literal (meta : & Meta) -> crate :: Result < Expr > { match meta { Meta :: Path (_) => Err (Error :: unsupported_format ("path") . with_span (meta)) , Meta :: List (_) => Err (Error :: unsupported_format ("list") . with_span (meta)) , Meta :: NameValue (nv) => Ok (nv . value . clone ()) , } }
};
}
