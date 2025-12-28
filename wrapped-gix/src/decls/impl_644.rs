macro_rules! deps {
    () => {
        Extensions!();
        Section!();
        Key!();
    };
}

macro_rules! impl_644 {
    () => {
        deps!();
        impl Section for Extensions { fn name (& self) -> & str { "extensions" } fn keys (& self) -> & [& dyn Key] { & [& Self :: OBJECT_FORMAT , & Self :: WORKTREE_CONFIG] } }
    };
}

impl_644!()