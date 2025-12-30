// Generated macro for ParseErrorKind (enum)
macro_rules! Depcrate_errorParseErrorKind {
() => {
// Module: crate::error
// Provides: {"ParseErrorKind"}
// Dependencies: {}
# [non_exhaustive] # [derive (Debug)] pub enum ParseErrorKind { UnterminatedString , UnexpectedChar (char) , UnexpectedToken { expected : & 'static str , found : & 'static str , } , IncompleteExpr (& 'static str) , UnterminatedExpression (String) , InvalidTarget (String) , }
};
}
