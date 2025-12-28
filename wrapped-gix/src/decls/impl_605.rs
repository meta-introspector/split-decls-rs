macro_rules! deps {
    () => {
        Key!();
        Committer!();
        Section!();
    };
}

macro_rules! impl_605 {
    () => {
        deps!();
        impl Section for Committer { fn name (& self) -> & str { "committer" } fn keys (& self) -> & [& dyn Key] { & [& Self :: NAME , & Self :: EMAIL] } }
    };
}

impl_605!()