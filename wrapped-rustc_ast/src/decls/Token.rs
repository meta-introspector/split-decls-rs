macro_rules! deps {
    () => {
        TokenKind!();
    };
}

macro_rules! Token {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct Token { pub kind : TokenKind , pub span : Span , }
    };
}

Token!()