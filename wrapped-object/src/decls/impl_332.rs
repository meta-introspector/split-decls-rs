macro_rules! deps {
    () => {
        ElfSymbolIterator!();
        ReadRef!();
        SymbolTable!();
        SymbolIndex!();
        FileHeader!();
        Endian!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > ElfSymbolIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) fn new (endian : Elf :: Endian , symbols : & 'file SymbolTable < 'data , Elf , R >) -> Self { ElfSymbolIterator { endian , symbols , index : SymbolIndex (1) , } } }
    };
}

impl_332!()