macro_rules! deps {
    () => {
        LiteralKind!();
        Literal!();
        HexLiteralKind!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Literal { # [doc = " If this literal was written as a `\\x` hex escape, then this returns"] # [doc = " the corresponding byte value. Otherwise, this returns `None`."] pub fn byte (& self) -> Option < u8 > { match self . kind { LiteralKind :: HexFixed (HexLiteralKind :: X) => { u8 :: try_from (self . c) . ok () } _ => None , } } }
    };
}

impl_72!()