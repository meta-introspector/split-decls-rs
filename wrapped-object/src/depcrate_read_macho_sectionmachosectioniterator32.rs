// Generated macro for MachOSectionIterator32 (type)
macro_rules! Depcrate_read_macho_sectionMachOSectionIterator32 {
() => {
// Module: crate::read::macho::section
// Provides: {"MachOSectionIterator32"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSectionIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSectionIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
