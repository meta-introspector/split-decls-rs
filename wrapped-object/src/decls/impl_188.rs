macro_rules! deps {
    () => {
        ArchiveSymbol!();
        ArchiveOffset!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < 'data > ArchiveSymbol < 'data > { # [doc = " Return the symbol name."] # [inline] pub fn name (& self) -> & 'data [u8] { self . name } # [doc = " Return the offset of the header for the member containing the symbol."] # [inline] pub fn offset (& self) -> ArchiveOffset { self . offset } }
    };
}

impl_188!()