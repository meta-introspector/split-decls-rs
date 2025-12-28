macro_rules! deps {
    () => {
        SymbolName!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl SymbolName { fn get_name (& self) -> Symbol { match self { SymbolName :: Link (s , _) | SymbolName :: Normal (s) => * s , } } }
    };
}

impl_220!();