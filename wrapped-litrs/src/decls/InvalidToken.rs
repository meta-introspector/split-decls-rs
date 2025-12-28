macro_rules! deps {
    () => {
        Span!();
        TokenKind!();
    };
}

macro_rules! InvalidToken {
    () => {
        deps!();
        # [doc = " An error signaling that a different kind of token was expected. Returned by"] # [doc = " the various `TryFrom` impls."] # [derive (Debug , Clone , Copy)] pub struct InvalidToken { pub (crate) expected : TokenKind , pub (crate) actual : TokenKind , pub (crate) span : Span , }
    };
}

InvalidToken!()