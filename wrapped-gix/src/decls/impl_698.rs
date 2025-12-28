macro_rules! deps {
    () => {
        Key!();
        Pack!();
        Section!();
    };
}

macro_rules! impl_698 {
    () => {
        deps!();
        impl Section for Pack { fn name (& self) -> & str { "pack" } fn keys (& self) -> & [& dyn Key] { & [& Self :: THREADS , & Self :: INDEX_VERSION] } }
    };
}

impl_698!();