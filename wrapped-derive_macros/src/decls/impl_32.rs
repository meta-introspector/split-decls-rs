macro_rules! deps {
    () => {
        DynamicAttr!();
        SvalAttribute!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl SvalAttribute for DynamicAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
    };
}

impl_32!()