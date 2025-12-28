macro_rules! deps {
    () => {
        Section!();
        Url!();
        Key!();
    };
}

macro_rules! impl_754 {
    () => {
        deps!();
        impl Section for Url { fn name (& self) -> & str { "url" } fn keys (& self) -> & [& dyn Key] { & [& Self :: INSTEAD_OF , & Self :: PUSH_INSTEAD_OF] } }
    };
}

impl_754!();