macro_rules! deps {
    () => {
        ImageRelocation!();
        SymbolIndex!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl pe :: ImageRelocation { # [doc = " Get the index of the symbol referenced by this relocation."] pub fn symbol (& self) -> SymbolIndex { SymbolIndex (self . symbol_table_index . get (LE) as usize) } }
    };
}

impl_255!();