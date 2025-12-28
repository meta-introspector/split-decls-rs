macro_rules! deps {
    () => {
        ByteLit!();
        CharLit!();
        CStringLit!();
        BoolLit!();
        TokenKind!();
        FloatLit!();
        StringLit!();
        IntegerLit!();
        Literal!();
        ByteStringLit!();
    };
}

macro_rules! kind_of {
    () => {
        deps!();
        fn kind_of (lit : & Literal < String >) -> TokenKind { match lit { Literal :: String (_) => TokenKind :: StringLit , Literal :: Bool (_) => TokenKind :: BoolLit , Literal :: Integer (_) => TokenKind :: IntegerLit , Literal :: Float (_) => TokenKind :: FloatLit , Literal :: Char (_) => TokenKind :: CharLit , Literal :: Byte (_) => TokenKind :: ByteLit , Literal :: ByteString (_) => TokenKind :: ByteStringLit , Literal :: CString (_) => TokenKind :: CStringLit , } }
    };
}

kind_of!();