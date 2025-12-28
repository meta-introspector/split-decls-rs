macro_rules! deps {
    () => {
        Endianness!();
        MachOSegmentIterator!();
        Endian!();
        MachHeader64!();
    };
}

macro_rules! MachOSegmentIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the segments in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSegmentIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSegmentIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOSegmentIterator64!()