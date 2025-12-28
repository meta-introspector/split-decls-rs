macro_rules! deps {
    () => {
        Item!();
        WasmSymbol!();
        SymbolIndex!();
        WasmSymbolIterator!();
    };
}

macro_rules! impl_767 {
    () => {
        deps!();
        impl < 'data , 'file > Iterator for WasmSymbolIterator < 'data , 'file > { type Item = WasmSymbol < 'data , 'file > ; fn next (& mut self) -> Option < Self :: Item > { let (index , symbol) = self . symbols . next () ? ; Some (WasmSymbol { index : SymbolIndex (index) , symbol , }) } }
    };
}

impl_767!()