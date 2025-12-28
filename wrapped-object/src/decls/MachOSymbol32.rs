macro_rules! deps {
    () => {
        Endian!();
        MachOSymbol!();
        Endianness!();
        MachHeader32!();
    };
}

macro_rules! MachOSymbol32 {
    () => {
        deps!();
        # [doc = " A symbol in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSymbol32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbol < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOSymbol32!()