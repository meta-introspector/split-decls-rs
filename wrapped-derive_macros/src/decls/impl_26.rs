macro_rules! deps {
    () => {
        UnlabeledVariantsAttr!();
        SvalAttribute!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl SvalAttribute for UnlabeledVariantsAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
    };
}

impl_26!();