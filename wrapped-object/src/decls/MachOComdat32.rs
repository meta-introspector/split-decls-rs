macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        MachOFile32!();
        MachOComdat!();
        MachHeader32!();
    };
}

macro_rules! MachOComdat32 {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`MachOFile32`]."] pub type MachOComdat32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdat < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOComdat32!()