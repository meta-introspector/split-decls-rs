macro_rules! deps {
    () => {
        Merge!();
        Key!();
        Section!();
    };
}

macro_rules! impl_689 {
    () => {
        deps!();
        impl Section for Merge { fn name (& self) -> & str { "merge" } fn keys (& self) -> & [& dyn Key] { & [& Self :: RENAME_LIMIT , # [cfg (feature = "merge")] & Self :: RENAMES , & Self :: RENORMALIZE , & Self :: DEFAULT , & Self :: DRIVER_NAME , & Self :: DRIVER_COMMAND , & Self :: DRIVER_RECURSIVE , # [cfg (feature = "merge")] & Self :: CONFLICT_STYLE ,] } }
    };
}

impl_689!()