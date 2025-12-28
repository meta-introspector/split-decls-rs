macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T : Internable + ? Sized > AsRef < T > for Interned < T > { # [inline] fn as_ref (& self) -> & T { & self . arc } }
    };
}

impl_38!()