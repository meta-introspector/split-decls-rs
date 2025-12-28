macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        FileHeader64!();
        ElfSymbolIterator!();
    };
}

macro_rules! ElfSymbolIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSymbolIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbolIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfSymbolIterator64!();