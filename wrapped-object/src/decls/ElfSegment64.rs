macro_rules! deps {
    () => {
        Endian!();
        ElfSegment!();
        Endianness!();
        FileHeader64!();
    };
}

macro_rules! ElfSegment64 {
    () => {
        deps!();
        # [doc = " A segment in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSegment64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSegment < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfSegment64!()