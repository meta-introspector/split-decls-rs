macro_rules! deps {
    () => {
        Key!();
        Section!();
        Safe!();
    };
}

macro_rules! impl_731 {
    () => {
        deps!();
        impl Section for Safe { fn name (& self) -> & str { "safe" } fn keys (& self) -> & [& dyn Key] { & [& Self :: DIRECTORY] } }
    };
}

impl_731!();