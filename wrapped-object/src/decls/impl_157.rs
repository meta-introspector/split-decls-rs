macro_rules! deps {
    () => {
        ReadRef!();
        SymbolTableInternal!();
        SymbolTable!();
        SymbolIterator!();
        SymbolIndex!();
        Result!();
        SymbolIteratorInternal!();
        SymbolInternal!();
        Symbol!();
        ObjectSymbolTable!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > ObjectSymbolTable < 'data > for SymbolTable < 'data , 'file , R > { type Symbol = Symbol < 'data , 'file , R > ; type SymbolIterator = SymbolIterator < 'data , 'file , R > ; fn symbols (& self) -> Self :: SymbolIterator { SymbolIterator { inner : map_inner ! (self . inner , SymbolTableInternal , SymbolIteratorInternal , | x | (x . 0 . symbols () , PhantomData)) , } } fn symbol_by_index (& self , index : SymbolIndex) -> Result < Self :: Symbol > { map_inner_option ! (self . inner , SymbolTableInternal , SymbolInternal , | x | x . 0 . symbol_by_index (index) . map (| x | (x , PhantomData))) . map (| inner | Symbol { inner }) } }
    };
}

impl_157!()