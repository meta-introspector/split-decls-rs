macro_rules! deps {
    () => {
        TransparentAttr!();
        SvalAttribute!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl SvalAttribute for TransparentAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } }
    };
}

impl_35!()