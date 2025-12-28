macro_rules! deps {
    () => {
        ByteStringLit!();
        CharLit!();
        BoolLit!();
        FloatLit!();
        ByteLit!();
        IntegerLit!();
        StringLit!();
        CStringLit!();
        Literal!();
    };
}

macro_rules! TokenKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum TokenKind { Punct , Ident , Group , Literal , BoolLit , ByteLit , ByteStringLit , CharLit , FloatLit , IntegerLit , StringLit , CStringLit , }
    };
}

TokenKind!()