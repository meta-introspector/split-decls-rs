macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        ElfSymbol!();
        FileHeader64!();
    };
}

macro_rules! ElfSymbol64 {
    () => {
        deps!();
        # [doc = " A symbol in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSymbol64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSymbol < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfSymbol64!()