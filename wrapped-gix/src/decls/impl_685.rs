macro_rules! deps {
    () => {
        Mailmap!();
        Key!();
        Section!();
    };
}

macro_rules! impl_685 {
    () => {
        deps!();
        impl Section for Mailmap { fn name (& self) -> & str { "mailmap" } fn keys (& self) -> & [& dyn Key] { & [& Self :: BLOB , & Self :: FILE] } }
    };
}

impl_685!();