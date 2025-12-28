macro_rules! deps {
    () => {
        ObjectSymbolTable!();
        ElfSymbolTable!();
        SymbolIterator!();
        Result!();
        ElfSymbolIterator!();
        SymbolIndex!();
        Symbol!();
        ElfSymbol!();
        FileHeader!();
        ReadRef!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < 'data , 'file , Elf : FileHeader , R : ReadRef < 'data > > ObjectSymbolTable < 'data > for ElfSymbolTable < 'data , 'file , Elf , R > { type Symbol = ElfSymbol < 'data , 'file , Elf , R > ; type SymbolIterator = ElfSymbolIterator < 'data , 'file , Elf , R > ; fn symbols (& self) -> Self :: SymbolIterator { ElfSymbolIterator :: new (self . endian , self . symbols) } fn symbol_by_index (& self , index : SymbolIndex) -> read :: Result < Self :: Symbol > { let symbol = self . symbols . symbol (index) ? ; Ok (ElfSymbol { endian : self . endian , symbols : self . symbols , index , symbol , }) } }
    };
}

impl_328!();