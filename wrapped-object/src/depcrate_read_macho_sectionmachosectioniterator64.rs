// Generated macro for MachOSectionIterator64 (type)
macro_rules! Depcrate_read_macho_sectionMachOSectionIterator64 {
() => {
// Module: crate::read::macho::section
// Provides: {"MachOSectionIterator64"}
// Dependencies: {}
# [doc = " An iterator for the sections in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSectionIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSectionIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
