// Generated macro for MachOComdatIterator32 (type)
macro_rules! Depcrate_read_macho_fileMachOComdatIterator32 {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOComdatIterator32"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`MachOFile64`]."] pub type MachOComdatIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdatIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
