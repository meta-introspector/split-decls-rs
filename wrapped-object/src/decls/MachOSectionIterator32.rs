macro_rules! deps {
    () => {
        Endian!();
        MachOSectionIterator!();
        Endianness!();
        MachHeader32!();
    };
}

macro_rules! MachOSectionIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSectionIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSectionIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOSectionIterator32!()