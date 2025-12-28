macro_rules! deps {
    () => {
        SymbolIndex!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl PartialEq for SymbolIndex { fn eq (& self , other : & SymbolIndex) -> bool { self . symbols == other . symbols } }
    };
}

impl_208!();