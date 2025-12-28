macro_rules! deps {
    () => {
        ElfSegmentIterator!();
        Endian!();
        FileHeader64!();
        Endianness!();
    };
}

macro_rules! ElfSegmentIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the segments in an [`ElfFile64`](super::ElfFile64)."] pub type ElfSegmentIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSegmentIterator < 'data , 'file , elf :: FileHeader64 < Endian > , R > ;
    };
}

ElfSegmentIterator64!();