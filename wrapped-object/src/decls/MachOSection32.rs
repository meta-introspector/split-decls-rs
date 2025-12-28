macro_rules! deps {
    () => {
        MachOSection!();
        MachHeader32!();
        Endianness!();
        Endian!();
    };
}

macro_rules! MachOSection32 {
    () => {
        deps!();
        # [doc = " A section in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSection32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSection < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOSection32!();