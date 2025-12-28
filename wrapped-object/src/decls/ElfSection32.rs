macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        FileHeader32!();
        ElfSection!();
    };
}

macro_rules! ElfSection32 {
    () => {
        deps!();
        # [doc = " A section in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSection32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSection < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfSection32!();