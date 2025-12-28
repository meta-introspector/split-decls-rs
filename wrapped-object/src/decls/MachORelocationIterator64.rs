macro_rules! deps {
    () => {
        Endian!();
        MachHeader64!();
        MachORelocationIterator!();
        Endianness!();
    };
}

macro_rules! MachORelocationIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the relocations in a [`MachOSection64`](super::MachOSection64)."] pub type MachORelocationIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachORelocationIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachORelocationIterator64!();