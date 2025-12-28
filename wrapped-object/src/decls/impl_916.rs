macro_rules! deps {
    () => {
        SymbolMapEntry!();
        ObjectMapEntry!();
    };
}

macro_rules! impl_916 {
    () => {
        deps!();
        impl < 'data > SymbolMapEntry for ObjectMapEntry < 'data > { # [inline] fn address (& self) -> u64 { self . address } }
    };
}

impl_916!()