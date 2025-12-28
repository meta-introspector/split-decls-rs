macro_rules! deps {
    () => {
        Endianness!();
        Endian!();
        MachOSegment!();
        MachHeader64!();
    };
}

macro_rules! MachOSegment64 {
    () => {
        deps!();
        # [doc = " A segment in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSegment64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSegment < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOSegment64!();