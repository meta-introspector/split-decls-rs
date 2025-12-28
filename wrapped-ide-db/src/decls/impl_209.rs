macro_rules! deps {
    () => {
        SymbolIndex!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl Eq for SymbolIndex { }
    };
}

impl_209!()