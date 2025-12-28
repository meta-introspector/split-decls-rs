macro_rules! deps {
    () => {
        MachOFile64!();
        MachOComdatSectionIterator!();
        Endian!();
        Endianness!();
        MachHeader64!();
    };
}

macro_rules! MachOComdatSectionIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`MachOFile64`]."] pub type MachOComdatSectionIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdatSectionIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOComdatSectionIterator64!();