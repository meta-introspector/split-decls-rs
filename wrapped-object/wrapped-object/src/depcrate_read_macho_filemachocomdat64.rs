// Generated macro for MachOComdat64 (type)
macro_rules! Depcrate_read_macho_fileMachOComdat64 {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOComdat64"}
// Dependencies: {}
# [doc = " A COMDAT section group in a [`MachOFile64`]."] pub type MachOComdat64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdat < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
