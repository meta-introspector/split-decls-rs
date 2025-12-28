macro_rules! deps {
    () => {
        ElfSymbolTable!();
        Endian!();
        Endianness!();
        FileHeader32!();
    };
}

macro_rules! ElfSymbolTable32 {
    () => {
        deps!();
        # [doc = " A symbol table in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSymbolTable32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbolTable < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfSymbolTable32!();