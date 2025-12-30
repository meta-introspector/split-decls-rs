// Generated macro for MachOComdatSectionIterator32 (type)
macro_rules! Depcrate_read_macho_fileMachOComdatSectionIterator32 {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOComdatSectionIterator32"}
// Dependencies: {}
# [doc = " An iterator for the sections in a COMDAT section group in a [`MachOFile32`]."] pub type MachOComdatSectionIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdatSectionIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
