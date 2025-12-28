macro_rules! deps {
    () => {
        CrateName!();
        CrateDisplayName!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl CrateDisplayName { pub fn canonical_name (& self) -> & Symbol { & self . canonical_name } pub fn crate_name (& self) -> & CrateName { & self . crate_name } }
    };
}

impl_29!()