macro_rules! deps {
    () => {
        SymbolMapName!();
    };
}

macro_rules! impl_910 {
    () => {
        deps!();
        impl < 'data > SymbolMapName < 'data > { # [doc = " Construct a `SymbolMapName`."] pub fn new (address : u64 , name : & 'data str) -> Self { SymbolMapName { address , name } } # [doc = " The symbol address."] # [inline] pub fn address (& self) -> u64 { self . address } # [doc = " The symbol name."] # [inline] pub fn name (& self) -> & 'data str { self . name } }
    };
}

impl_910!();