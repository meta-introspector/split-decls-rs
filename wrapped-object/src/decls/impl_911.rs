macro_rules! deps {
    () => {
        SymbolMapEntry!();
        SymbolMapName!();
    };
}

macro_rules! impl_911 {
    () => {
        deps!();
        impl < 'data > SymbolMapEntry for SymbolMapName < 'data > { # [inline] fn address (& self) -> u64 { self . address } }
    };
}

impl_911!();