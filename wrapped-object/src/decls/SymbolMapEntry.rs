macro_rules! deps {
    () => {
        SymbolMap!();
    };
}

macro_rules! SymbolMapEntry {
    () => {
        deps!();
        # [doc = " An entry in a [`SymbolMap`]."] pub trait SymbolMapEntry { # [doc = " The symbol address."] fn address (& self) -> u64 ; }
    };
}

SymbolMapEntry!()