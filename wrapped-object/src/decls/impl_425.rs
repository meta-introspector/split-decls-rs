macro_rules! deps {
    () => {
        VersionIndex!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl VersionIndex { # [doc = " Return the version index."] pub fn index (& self) -> u16 { self . 0 & elf :: VERSYM_VERSION } # [doc = " Return true if it is the local index."] pub fn is_local (& self) -> bool { self . index () == elf :: VER_NDX_LOCAL } # [doc = " Return true if it is the global index."] pub fn is_global (& self) -> bool { self . index () == elf :: VER_NDX_GLOBAL } # [doc = " Return the hidden flag."] pub fn is_hidden (& self) -> bool { self . 0 & elf :: VERSYM_HIDDEN != 0 } }
    };
}

impl_425!()