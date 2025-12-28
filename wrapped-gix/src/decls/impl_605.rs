macro_rules! deps {
    () => {
        Committer!();
        Key!();
        Section!();
    };
}

macro_rules! impl_605 {
    () => {
        deps!();
        impl Section for Committer { fn name (& self) -> & str { "committer" } fn keys (& self) -> & [& dyn Key] { & [& Self :: NAME , & Self :: EMAIL] } }
    };
}

impl_605!();