macro_rules! deps {
    () => {
        Id!();
        SymbolId!();
    };
}

macro_rules! impl_1130 {
    () => {
        deps!();
        impl < const DYNAMIC : bool > Id for SymbolId < DYNAMIC > { fn index (& self) -> usize { self . 0 } }
    };
}

impl_1130!();