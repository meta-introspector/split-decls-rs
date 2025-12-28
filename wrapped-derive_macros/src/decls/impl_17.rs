macro_rules! deps {
    () => {
        SvalAttribute!();
        SkipAttr!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl SvalAttribute for SkipAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
    };
}

impl_17!()