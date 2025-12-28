macro_rules! deps {
    () => {
        Endian!();
        Endianness!();
        FileHeader32!();
        ElfSymbolIterator!();
    };
}

macro_rules! ElfSymbolIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSymbolIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbolIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfSymbolIterator32!()