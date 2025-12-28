macro_rules! deps {
    () => {
        MachOSymbolIterator!();
        MachHeader64!();
        Endian!();
        Endianness!();
    };
}

macro_rules! MachOSymbolIterator64 {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSymbolIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbolIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOSymbolIterator64!();