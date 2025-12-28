macro_rules! deps {
    () => {
        FileHeader!();
        ReadRef!();
        Symbol!();
        Item!();
        SymbolIndex!();
        SymbolIterator!();
    };
}

macro_rules! impl_809 {
    () => {
        deps!();
        impl < 'data , 'table , Xcoff : FileHeader , R : ReadRef < 'data > > Iterator for SymbolIterator < 'data , 'table , Xcoff , R > { type Item = (SymbolIndex , & 'data Xcoff :: Symbol) ; fn next (& mut self) -> Option < Self :: Item > { loop { let index = SymbolIndex (self . index) ; let symbol = self . symbols . symbol_unchecked (index) . ok () ? ; self . index += 1 + symbol . n_numaux () as usize ; if ! symbol . is_null () { return Some ((index , symbol)) ; } } } }
    };
}

impl_809!()