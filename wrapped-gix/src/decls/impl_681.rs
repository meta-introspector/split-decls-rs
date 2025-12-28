macro_rules! deps {
    () => {
        Section!();
        Init!();
        Key!();
    };
}

macro_rules! impl_681 {
    () => {
        deps!();
        impl Section for Init { fn name (& self) -> & str { "init" } fn keys (& self) -> & [& dyn Key] { & [& Self :: DEFAULT_BRANCH] } }
    };
}

impl_681!();