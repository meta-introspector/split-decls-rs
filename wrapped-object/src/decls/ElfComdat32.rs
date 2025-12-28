macro_rules! deps {
    () => {
        Endianness!();
        FileHeader32!();
        ElfComdat!();
        Endian!();
    };
}

macro_rules! ElfComdat32 {
    () => {
        deps!();
        # [doc = " A COMDAT section group in an [`ElfFile32`](super::ElfFile32)."] pub type ElfComdat32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfComdat < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfComdat32!();