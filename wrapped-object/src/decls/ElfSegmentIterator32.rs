macro_rules! deps {
    () => {
        ElfSegmentIterator!();
        FileHeader32!();
        Endianness!();
        Endian!();
    };
}

macro_rules! ElfSegmentIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the segments in an [`ElfFile32`](super::ElfFile32)."] pub type ElfSegmentIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = ElfSegmentIterator < 'data , 'file , elf :: FileHeader32 < Endian > , R > ;
    };
}

ElfSegmentIterator32!();