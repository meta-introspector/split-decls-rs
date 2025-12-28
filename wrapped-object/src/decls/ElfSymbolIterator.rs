macro_rules! deps {
    () => {
        ReadRef!();
        SymbolTable!();
        Endian!();
        SymbolIndex!();
        FileHeader!();
    };
}

macro_rules! ElfSymbolIterator {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in an [`ElfFile`](super::ElfFile)."] pub struct ElfSymbolIterator < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { endian : Elf :: Endian , symbols : & 'file SymbolTable < 'data , Elf , R > , index : SymbolIndex , }
    };
}

ElfSymbolIterator!()