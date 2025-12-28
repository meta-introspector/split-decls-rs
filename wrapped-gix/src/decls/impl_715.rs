macro_rules! deps {
    () => {
        Push!();
        Section!();
        Key!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl Section for Push { fn name (& self) -> & str { "push" } fn keys (& self) -> & [& dyn Key] { & [& Self :: DEFAULT] } }
    };
}

impl_715!()