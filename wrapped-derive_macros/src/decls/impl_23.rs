macro_rules! deps {
    () => {
        UnindexedFieldsAttr!();
        SvalAttribute!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl SvalAttribute for UnindexedFieldsAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
    };
}

impl_23!();