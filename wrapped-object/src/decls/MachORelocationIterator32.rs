macro_rules! deps {
    () => {
        Endianness!();
        MachORelocationIterator!();
        MachHeader32!();
        Endian!();
    };
}

macro_rules! MachORelocationIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the relocations in a [`MachOSection32`](super::MachOSection32)."] pub type MachORelocationIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachORelocationIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachORelocationIterator32!()