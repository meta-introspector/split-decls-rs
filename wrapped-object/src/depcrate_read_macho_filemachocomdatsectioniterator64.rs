// Generated macro for MachOComdatSectionIterator64 (type)
macro_rules! Depcrate_read_macho_fileMachOComdatSectionIterator64 {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOComdatSectionIterator64"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`MachOFile64`]."] pub type MachOComdatSectionIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdatSectionIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
