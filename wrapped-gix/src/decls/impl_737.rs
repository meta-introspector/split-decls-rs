macro_rules! deps {
    () => {
        Section!();
        Ssh!();
        Key!();
    };
}

macro_rules! impl_737 {
    () => {
        deps!();
        impl Section for Ssh { fn name (& self) -> & str { "ssh" } fn keys (& self) -> & [& dyn Key] { & [& Self :: VARIANT] } }
    };
}

impl_737!()