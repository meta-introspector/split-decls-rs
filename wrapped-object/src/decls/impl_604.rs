macro_rules! deps {
    () => {
        MachOSymbolIterator!();
        SymbolIndex!();
        MachOSymbolTable!();
        Result!();
        Symbol!();
        MachOSymbol!();
        ReadRef!();
        ObjectSymbolTable!();
        SymbolIterator!();
        MachHeader!();
    };
}

macro_rules! impl_604 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > ObjectSymbolTable < 'data > for MachOSymbolTable < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Symbol = MachOSymbol < 'data , 'file , Mach , R > ; type SymbolIterator = MachOSymbolIterator < 'data , 'file , Mach , R > ; fn symbols (& self) -> Self :: SymbolIterator { MachOSymbolIterator :: new (self . file) } fn symbol_by_index (& self , index : SymbolIndex) -> Result < Self :: Symbol > { let nlist = self . file . symbols . symbol (index) ? ; MachOSymbol :: new (self . file , index , nlist) . read_error ("Unsupported Mach-O symbol index") } }
    };
}

impl_604!()