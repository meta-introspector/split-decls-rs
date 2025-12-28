macro_rules! deps {
    () => {
        Endian!();
        MachOSymbolTable!();
        Endianness!();
        MachHeader64!();
    };
}

macro_rules! MachOSymbolTable64 {
    () => {
        deps!();
        # [doc = " A symbol table in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSymbolTable64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbolTable < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOSymbolTable64!();