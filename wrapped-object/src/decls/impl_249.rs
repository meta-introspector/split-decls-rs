macro_rules! deps {
    () => {
        SymbolIndex!();
        ImageAuxSymbolWeak!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl pe :: ImageAuxSymbolWeak { # [doc = " Get the symbol index of the default definition."] pub fn default_symbol (& self) -> SymbolIndex { SymbolIndex (self . weak_default_sym_index . get (LE) as usize) } }
    };
}

impl_249!()