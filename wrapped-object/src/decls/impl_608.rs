macro_rules! deps {
    () => {
        MachOFile!();
        MachOSymbolIterator!();
        ReadRef!();
        SymbolIndex!();
        MachHeader!();
    };
}

macro_rules! impl_608 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > MachOSymbolIterator < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) fn new (file : & 'file MachOFile < 'data , Mach , R >) -> Self { MachOSymbolIterator { file , index : SymbolIndex (0) , } } pub (super) fn empty (file : & 'file MachOFile < 'data , Mach , R >) -> Self { MachOSymbolIterator { file , index : SymbolIndex (file . symbols . len ()) , } } }
    };
}

impl_608!();