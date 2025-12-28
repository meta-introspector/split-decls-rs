macro_rules! deps {
    () => {
        Version!();
        Mode!();
        BytesToEntriesIter!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        # [doc = " Access"] impl < BR > BytesToEntriesIter < BR > { # [doc = " The pack version currently being iterated"] pub fn version (& self) -> crate :: data :: Version { self . version } # [doc = " The kind of iteration"] pub fn mode (& self) -> input :: Mode { self . mode } }
    };
}

impl_142!();