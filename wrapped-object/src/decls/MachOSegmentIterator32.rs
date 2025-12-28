macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        MachHeader32!();
        MachOSegmentIterator!();
    };
}

macro_rules! MachOSegmentIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the segments in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSegmentIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSegmentIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOSegmentIterator32!()