macro_rules! ParseErrorKind {
    () => {
        # [non_exhaustive] # [derive (Debug)] pub enum ParseErrorKind { UnterminatedString , UnexpectedChar (char) , UnexpectedToken { expected : & 'static str , found : & 'static str , } , IncompleteExpr (& 'static str) , UnterminatedExpression (String) , InvalidTarget (String) , }
    };
}

ParseErrorKind!();