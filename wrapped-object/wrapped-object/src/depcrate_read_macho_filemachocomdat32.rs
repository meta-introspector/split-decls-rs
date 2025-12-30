// Generated macro for MachOComdat32 (type)
macro_rules! Depcrate_read_macho_fileMachOComdat32 {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOComdat32"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`MachOFile32`]."] pub type MachOComdat32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdat < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
