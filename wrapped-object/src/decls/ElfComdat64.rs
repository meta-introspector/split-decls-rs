macro_rules! deps {
    () => {
        ElfComdat!();
        Endian!();
        Endianness!();
        FileHeader64!();
    };
}

macro_rules! ElfComdat64 {
    () => {
        deps!();
        # [doc = " A COMDAT section group in an [`ElfFile64`](super::ElfFile64)."] pub type ElfComdat64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdat < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfComdat64!()