macro_rules! deps {
    () => {
        Endianness!();
        MachHeader32!();
        MachOSymbolIterator!();
        Endian!();
    };
}

macro_rules! MachOSymbolIterator32 {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSymbolIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbolIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOSymbolIterator32!();