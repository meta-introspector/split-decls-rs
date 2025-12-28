macro_rules! deps {
    () => {
        MachHeader64!();
        Endian!();
        Endianness!();
        MachOSymbol!();
    };
}

macro_rules! MachOSymbol64 {
    () => {
        deps!();
        # [doc = " A symbol in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSymbol64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbol < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOSymbol64!();