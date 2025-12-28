macro_rules! deps {
    () => {
        Endian!();
        ElfSymbol!();
        FileHeader32!();
        Endianness!();
    };
}

macro_rules! ElfSymbol32 {
    () => {
        deps!();
        # [doc = " A symbol in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSymbol32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbol < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfSymbol32!();