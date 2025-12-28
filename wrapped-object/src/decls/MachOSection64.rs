macro_rules! deps {
    () => {
        Endian!();
        Endianness!();
        MachOSection!();
        MachHeader64!();
    };
}

macro_rules! MachOSection64 {
    () => {
        deps!();
        # [doc = " A section in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSection64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSection < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOSection64!()