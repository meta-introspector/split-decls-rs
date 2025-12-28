macro_rules! deps {
    () => {
        CoffHeader!();
        CoffSymbol!();
        ReadRef!();
        Item!();
        CoffSymbolIterator!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for CoffSymbolIterator < 'data , 'file , R , Coff > { type Item = CoffSymbol < 'data , 'file , R , Coff > ; fn next (& mut self) -> Option < Self :: Item > { let index = self . index ; let symbol = self . file . symbols . symbol (index) . ok () ? ; self . index . 0 += 1 + symbol . number_of_aux_symbols () as usize ; Some (CoffSymbol { file : self . file , index , symbol , }) } }
    };
}

impl_240!();