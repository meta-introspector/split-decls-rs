macro_rules! deps {
    () => {
        MachOComdatSectionIterator!();
        MachHeader32!();
        Endianness!();
        MachOFile32!();
        Endian!();
    };
}

macro_rules! MachOComdatSectionIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`MachOFile32`]."] pub type MachOComdatSectionIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdatSectionIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOComdatSectionIterator32!();