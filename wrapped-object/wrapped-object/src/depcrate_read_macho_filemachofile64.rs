// Generated macro for MachOFile64 (type)
macro_rules! Depcrate_read_macho_fileMachOFile64 {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOFile64"}
// Dependencies: {}
# [doc = " A 64-bit Mach-O object file."] # [doc = ""] # [doc = " This is a file that starts with [`macho::MachHeader64`], and corresponds"] # [doc = " to [`crate::FileKind::MachO64`]."] pub type MachOFile64 < 'data , Endian = Endianness , R = & 'data [u8] > = MachOFile < 'data , macho :: MachHeader64 < Endian > , R > ;
};
}
