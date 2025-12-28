macro_rules! deps {
    () => {
        SymbolId!();
    };
}

macro_rules! impl_1131 {
    () => {
        deps!();
        impl < const DYNAMIC : bool > IdPrivate for SymbolId < DYNAMIC > { fn new (id : usize) -> Self { SymbolId (id) } }
    };
}

impl_1131!()