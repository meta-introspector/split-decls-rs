macro_rules! deps {
    () => {
        MachOSection!();
        Endian!();
        MachHeader32!();
        Endianness!();
    };
}

macro_rules! MachOSection32 {
    () => {
        deps!();
        # [doc = " A section in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSection32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSection < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOSection32!()