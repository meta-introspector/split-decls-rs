macro_rules! deps {
    () => {
        MachOComdatIterator!();
        MachOFile64!();
        Endianness!();
        Endian!();
        MachHeader32!();
    };
}

macro_rules! MachOComdatIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`MachOFile64`]."] pub type MachOComdatIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdatIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOComdatIterator32!();