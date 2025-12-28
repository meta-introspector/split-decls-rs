macro_rules! deps {
    () => {
        SvalAttribute!();
        UnindexedVariantsAttr!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl SvalAttribute for UnindexedVariantsAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
    };
}

impl_29!()