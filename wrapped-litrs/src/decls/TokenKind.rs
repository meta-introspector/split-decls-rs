macro_rules! deps {
    () => {
        CharLit!();
        CStringLit!();
        Literal!();
        ByteLit!();
        FloatLit!();
        StringLit!();
        IntegerLit!();
        ByteStringLit!();
        BoolLit!();
    };
}

macro_rules! TokenKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum TokenKind { Punct , Ident , Group , Literal , BoolLit , ByteLit , ByteStringLit , CharLit , FloatLit , IntegerLit , StringLit , CStringLit , }
    };
}

TokenKind!();