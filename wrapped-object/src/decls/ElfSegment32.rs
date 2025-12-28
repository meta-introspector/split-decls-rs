macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        ElfSegment!();
        FileHeader32!();
    };
}

macro_rules! ElfSegment32 {
    () => {
        deps!();
        # [doc = " A segment in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSegment32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSegment < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfSegment32!();