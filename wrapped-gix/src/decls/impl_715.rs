macro_rules! deps {
    () => {
        Key!();
        Section!();
        Push!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl Section for Push { fn name (& self) -> & str { "push" } fn keys (& self) -> & [& dyn Key] { & [& Self :: DEFAULT] } }
    };
}

impl_715!();