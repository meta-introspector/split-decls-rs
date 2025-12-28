macro_rules! deps {
    () => {
        ReadRef!();
        CoffHeader!();
        SymbolIndex!();
        SymbolIterator!();
        ImageSymbol!();
        Item!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl < 'data , 'table , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for SymbolIterator < 'data , 'table , R , Coff > { type Item = (SymbolIndex , & 'data Coff :: ImageSymbol) ; fn next (& mut self) -> Option < Self :: Item > { let index = self . index ; let symbol = self . symbols . symbol (index) . ok () ? ; self . index . 0 += 1 + symbol . number_of_aux_symbols () as usize ; Some ((index , symbol)) } }
    };
}

impl_231!()