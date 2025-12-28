macro_rules! deps {
    () => {
        Endianness!();
        MachOSymbolTable!();
        Endian!();
        MachHeader32!();
    };
}

macro_rules! MachOSymbolTable32 {
    () => {
        deps!();
        # [doc = " A symbol table in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSymbolTable32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbolTable < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOSymbolTable32!()