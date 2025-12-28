macro_rules! deps {
    () => {
        SymbolId!();
        Id!();
        Symbol!();
        Item!();
    };
}

macro_rules! impl_1133 {
    () => {
        deps!();
        impl < 'data , const DYNAMIC : bool > Item for Symbol < 'data , DYNAMIC > { type Id = SymbolId < DYNAMIC > ; fn is_deleted (& self) -> bool { self . delete } }
    };
}

impl_1133!()