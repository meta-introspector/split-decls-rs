macro_rules! deps {
    () => {
        FlattenAttr!();
        SvalAttribute!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl SvalAttribute for FlattenAttr { type Result = bool ; fn from_lit (& self , lit : & Lit) -> Self :: Result { # [cfg (not (feature = "flatten"))] { let _ = lit ; panic ! ("the `flatten` attribute can only be used when the `flatten` Cargo feature of `sval_derive` is enabled") ; } # [cfg (feature = "flatten")] { if let Lit :: Bool (ref b) = lit { b . value } else { panic ! ("unexpected value") } } } }
    };
}

impl_38!()