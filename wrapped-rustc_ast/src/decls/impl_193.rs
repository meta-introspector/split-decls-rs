macro_rules! deps {
    () => {
        Safety!();
        Path!();
        AttrItem!();
        NormalAttr!();
        AttrArgs!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl NormalAttr { pub fn from_ident (ident : Ident) -> Self { Self { item : AttrItem { unsafety : Safety :: Default , path : Path :: from_ident (ident) , args : AttrArgs :: Empty , tokens : None , } , tokens : None , } } }
    };
}

impl_193!();