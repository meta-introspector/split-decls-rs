macro_rules! deps {
    () => {
        MachOFile64!();
        Endianness!();
        MachHeader64!();
        Endian!();
        MachOComdat!();
    };
}

macro_rules! MachOComdat64 {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`MachOFile64`]."] pub type MachOComdat64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdat < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOComdat64!();