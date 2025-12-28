macro_rules! deps {
    () => {
        MachHeader64!();
        Endianness!();
        MachOSectionIterator!();
        Endian!();
    };
}

macro_rules! MachOSectionIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSectionIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSectionIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOSectionIterator64!()