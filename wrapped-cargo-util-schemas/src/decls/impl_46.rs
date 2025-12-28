macro_rules! deps {
    () => {
        TomlLockfilePatch!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl TomlLockfilePatch { fn is_empty (& self) -> bool { self . unused . is_empty () } }
    };
}

impl_46!();