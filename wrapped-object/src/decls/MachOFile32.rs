macro_rules! deps {
    () => {
        MachOFile!();
        Endian!();
        Endianness!();
        MachHeader32!();
    };
}

macro_rules! MachOFile32 {
    () => {
        deps!();
        # [doc = " A 32-bit Mach-O object file."] # [doc = ""] # [doc = " This is a file that starts with [`macho::MachHeader32`], and corresponds"] # [doc = " to [`crate::FileKind::MachO32`]."] pub type MachOFile32 < 'data , Endian = Endianness , R = & 'data [u8] > = MachOFile < 'data , macho :: MachHeader32 < Endian > , R > ;
    };
}

MachOFile32!();