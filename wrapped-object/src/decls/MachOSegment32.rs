macro_rules! deps {
    () => {
        MachHeader32!();
        Endian!();
        Endianness!();
        MachOSegment!();
    };
}

macro_rules! MachOSegment32 {
    () => {
        deps!();
        # [doc = " A segment in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSegment32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSegment < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOSegment32!();