macro_rules! deps {
    () => {
        Endianness!();
        MachHeader64!();
        Endian!();
        MachOFile!();
    };
}

macro_rules! MachOFile64 {
    () => {
        deps!();
        # [doc = " A 64-bit Mach-O object file."] # [doc = ""] # [doc = " This is a file that starts with [`macho::MachHeader64`], and corresponds"] # [doc = " to [`crate::FileKind::MachO64`]."] pub type MachOFile64 < 'data , Endian = Endianness , R = & 'data [u8] > = MachOFile < 'data , macho :: MachHeader64 < Endian > , R > ;
    };
}

MachOFile64!()