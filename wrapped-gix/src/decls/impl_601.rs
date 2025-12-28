macro_rules! deps {
    () => {
        Key!();
        Clone!();
        Section!();
    };
}

macro_rules! impl_601 {
    () => {
        deps!();
        impl Section for Clone { fn name (& self) -> & str { "clone" } fn keys (& self) -> & [& dyn Key] { & [& Self :: DEFAULT_REMOTE_NAME , & Self :: REJECT_SHALLOW] } }
    };
}

impl_601!();