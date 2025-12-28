macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        MachHeader64!();
        MachOComdatIterator!();
        MachOFile64!();
    };
}

macro_rules! MachOComdatIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`MachOFile64`]."] pub type MachOComdatIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdatIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOComdatIterator64!();