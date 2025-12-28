macro_rules! deps {
    () => {
        Key!();
        Index!();
        Section!();
    };
}

macro_rules! impl_676 {
    () => {
        deps!();
        impl Section for Index { fn name (& self) -> & str { "index" } fn keys (& self) -> & [& dyn Key] { & [& Self :: THREADS , & Self :: SKIP_HASH] } }
    };
}

impl_676!()