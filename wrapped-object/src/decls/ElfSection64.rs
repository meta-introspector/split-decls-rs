macro_rules! deps {
    () => {
        Endianness!();
        FileHeader64!();
        ElfSection!();
        Endian!();
    };
}

macro_rules! ElfSection64 {
    () => {
        deps!();
        # [doc = " A section in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSection64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSection < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfSection64!()