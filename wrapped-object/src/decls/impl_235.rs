macro_rules! deps {
    () => {
        Result!();
        SymbolIterator!();
        ObjectSymbolTable!();
        CoffHeader!();
        ReadRef!();
        CoffSymbol!();
        Symbol!();
        CoffSymbolIterator!();
        CoffSymbolTable!();
        SymbolIndex!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > ObjectSymbolTable < 'data > for CoffSymbolTable < 'data , 'file , R , Coff > { type Symbol = CoffSymbol < 'data , 'file , R , Coff > ; type SymbolIterator = CoffSymbolIterator < 'data , 'file , R , Coff > ; fn symbols (& self) -> Self :: SymbolIterator { CoffSymbolIterator :: new (self . file) } fn symbol_by_index (& self , index : SymbolIndex) -> Result < Self :: Symbol > { let symbol = self . file . symbols . symbol (index) ? ; Ok (CoffSymbol { file : self . file , index , symbol , }) } }
    };
}

impl_235!()