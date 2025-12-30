// Generated macro for invalid_token_display (function)
macro_rules! Depcrate_testsinvalid_token_display {
() => {
// Module: crate::tests
// Provides: {"invalid_token_display"}
// Dependencies: {}
# [cfg (feature = "proc-macro2")] # [test] fn invalid_token_display () { use crate :: { err :: TokenKind , InvalidToken } ; let span = crate :: err :: Span :: Two (proc_macro2 :: Span :: call_site ()) ; assert_eq ! (InvalidToken { actual : TokenKind :: StringLit , expected : TokenKind :: FloatLit , span , } . to_string () , r#"expected a float literal (e.g. `3.14`), but found a string literal (e.g. "Ferris")"# ,) ; assert_eq ! (InvalidToken { actual : TokenKind :: Punct , expected : TokenKind :: Literal , span , } . to_string () , r#"expected a literal, but found a punctuation character"# ,) ; }
};
}
