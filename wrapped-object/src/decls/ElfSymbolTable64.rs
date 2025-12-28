macro_rules! deps {
    () => {
        Endian!();
        Endianness!();
        ElfSymbolTable!();
        FileHeader64!();
    };
}

macro_rules! ElfSymbolTable64 {
    () => {
        deps!();
        # [doc = " A symbol table in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSymbolTable64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbolTable < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfSymbolTable64!();