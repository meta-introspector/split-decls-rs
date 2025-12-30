// Generated macro for MachOSection64 (type)
macro_rules! Depcrate_read_macho_sectionMachOSection64 {
() => {
// Module: crate::read::macho::section
// Provides: {"MachOSection64"}
// Dependencies: {}
# [doc = " A section in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSection64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSection < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
