// Generated macro for Token (enum)
macro_rules! Depcrate_utilsToken {
() => {
// Module: crate::utils
// Provides: {"Token"}
// Dependencies: {}
# [derive (Clone , Copy)] pub enum Token < 'a > { # [doc = " Matches any number of comments / doc comments."] AnyComment , Ident (& 'a str) , CaptureIdent , LitStr , CaptureLitStr , Bang , CloseBrace , CloseBracket , CloseParen , # [doc = " This will consume the first colon even if the second doesn't exist."] DoubleColon , Comma , Eq , Lifetime , Lt , Gt , OpenBrace , OpenBracket , OpenParen , Pound , Semi , }
};
}
