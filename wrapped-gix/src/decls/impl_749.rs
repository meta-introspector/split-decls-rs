macro_rules! deps {
    () => {
        Section!();
        User!();
        Key!();
    };
}

macro_rules! impl_749 {
    () => {
        deps!();
        impl Section for User { fn name (& self) -> & str { "user" } fn keys (& self) -> & [& dyn Key] { & [& Self :: NAME , & Self :: EMAIL] } }
    };
}

impl_749!()