macro_rules! deps {
    () => {
        UnlabeledFieldsAttr!();
        SvalAttribute!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl SvalAttribute for UnlabeledFieldsAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
    };
}

impl_20!();