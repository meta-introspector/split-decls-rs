// Generated macro for MachOSection32 (type)
macro_rules! Depcrate_read_macho_sectionMachOSection32 {
() => {
// Module: crate::read::macho::section
// Provides: {"MachOSection32"}
// Dependencies: {}
# [doc = " A section in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSection32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSection < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
