// Generated macro for impl_39 (impl)
macro_rules! Depcrate_errorimpl_39 {
() => {
// Module: crate::error
// Provides: {"impl_39"}
// Dependencies: {}
impl fmt :: Display for ParseErrorKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use ParseErrorKind :: * ; match self { UnterminatedString => write ! (f , "unterminated string in cfg") , UnexpectedChar (ch) => write ! (f , "unexpected character `{}` in cfg, expected parens, a comma, \
                 an identifier, or a string" , ch) , UnexpectedToken { expected , found } => { write ! (f , "expected {}, found {}" , expected , found) } IncompleteExpr (expected) => { write ! (f , "expected {}, but cfg expression ended" , expected) } UnterminatedExpression (s) => { write ! (f , "unexpected content `{}` found after cfg expression" , s) } InvalidTarget (s) => write ! (f , "invalid target specifier: {}" , s) , } } }
};
}
