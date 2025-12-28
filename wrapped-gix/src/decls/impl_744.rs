macro_rules! deps {
    () => {
        Section!();
        Status!();
        Key!();
    };
}

macro_rules! impl_744 {
    () => {
        deps!();
        impl Section for Status { fn name (& self) -> & str { "status" } fn keys (& self) -> & [& dyn Key] { & [& Self :: SHOW_UNTRACKED_FILES , & Self :: RENAMES , & Self :: RENAME_LIMIT] } }
    };
}

impl_744!()