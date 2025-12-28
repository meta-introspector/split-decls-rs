macro_rules! deps {
    () => {
        FileHeader!();
        ObjectSymbolTable!();
        XcoffSymbolIterator!();
        Result!();
        Symbol!();
        ReadRef!();
        XcoffSymbol!();
        SymbolIndex!();
        XcoffSymbolTable!();
        SymbolIterator!();
    };
}

macro_rules! impl_814 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > ObjectSymbolTable < 'data > for XcoffSymbolTable < 'data , 'file , Xcoff , R > { type Symbol = XcoffSymbol < 'data , 'file , Xcoff , R > ; type SymbolIterator = XcoffSymbolIterator < 'data , 'file , Xcoff , R > ; fn symbols (& self) -> Self :: SymbolIterator { XcoffSymbolIterator { file : self . file , symbols : self . symbols . iter () , } } fn symbol_by_index (& self , index : SymbolIndex) -> read :: Result < Self :: Symbol > { let symbol = self . symbols . symbol (index) ? ; Ok (XcoffSymbol { file : self . file , symbols : self . symbols , index , symbol , }) } }
    };
}

impl_814!();