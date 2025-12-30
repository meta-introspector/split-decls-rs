// Generated macro for MachOFile32 (type)
macro_rules! Depcrate_read_macho_fileMachOFile32 {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOFile32"}
// Dependencies: {}
# [doc = " A 32-bit Mach-O object file."] # [doc = ""] # [doc = " This is a file that starts with [`macho::MachHeader32`], and corresponds"] # [doc = " to [`crate::FileKind::MachO32`]."] pub type MachOFile32 < 'data , Endian = Endianness , R = & 'data [u8] > = MachOFile < 'data , macho :: MachHeader32 < Endian > , R > ;
};
}
