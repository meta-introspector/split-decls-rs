macro_rules! deps {
    () => {
        Section!();
        Key!();
        Diff!();
    };
}

macro_rules! impl_630 {
    () => {
        deps!();
        impl Section for Diff { fn name (& self) -> & str { "diff" } fn keys (& self) -> & [& dyn Key] { & [& Self :: ALGORITHM , & Self :: IGNORE_SUBMODULES , & Self :: RENAME_LIMIT , & Self :: RENAMES , & Self :: DRIVER_COMMAND , & Self :: DRIVER_TEXTCONV , & Self :: DRIVER_ALGORITHM , & Self :: DRIVER_BINARY , & Self :: EXTERNAL ,] } }
    };
}

impl_630!();