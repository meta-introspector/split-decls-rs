macro_rules! deps {
    () => {
        Item!();
        MachOSymbolIterator!();
        MachHeader!();
        ReadRef!();
        MachOSymbol!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > Iterator for MachOSymbolIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { type Item = MachOSymbol < 'data , 'file , Mach , R > ; fn next (& mut self) -> Option < Self :: Item > { loop { let index = self . index ; let nlist = self . file . symbols . symbols . get (index . 0) ? ; self . index . 0 += 1 ; if let Some (symbol) = MachOSymbol :: new (self . file , index , nlist) { return Some (symbol) ; } } } }
    };
}

impl_610!();