macro_rules! deps {
    () => {
        TokenKind!();
        InvalidToken!();
        ByteStringLit!();
        CharLit!();
        ByteLit!();
        CStringLit!();
        IntegerLit!();
        BoolLit!();
        StringLit!();
        Literal!();
        FloatLit!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl fmt :: Display for InvalidToken { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fn kind_desc (kind : TokenKind) -> & 'static str { match kind { TokenKind :: Punct => "a punctuation character" , TokenKind :: Ident => "an identifier" , TokenKind :: Group => "a group" , TokenKind :: Literal => "a literal" , TokenKind :: BoolLit => "a bool literal (`true` or `false`)" , TokenKind :: ByteLit => "a byte literal (e.g. `b'r')" , TokenKind :: ByteStringLit => r#"a byte string literal (e.g. `b"fox"`)"# , TokenKind :: CharLit => "a character literal (e.g. `'P'`)" , TokenKind :: FloatLit => "a float literal (e.g. `3.14`)" , TokenKind :: IntegerLit => "an integer literal (e.g. `27`)" , TokenKind :: StringLit => r#"a string literal (e.g. "Ferris")"# , TokenKind :: CStringLit => r#"a C string literal (e.g. c"Ferris")"# , } } write ! (f , "expected {}, but found {}" , kind_desc (self . expected) , kind_desc (self . actual)) } }
    };
}

impl_104!()