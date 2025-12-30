// Generated macro for MachOComdatIterator64 (type)
macro_rules! Depcrate_read_macho_fileMachOComdatIterator64 {
() => {
// Module: crate::read::macho::file
// Provides: {"MachOComdatIterator64"}
// Dependencies: {}
# [doc = " An iterator for the COMDAT section groups in a [`MachOFile64`]."] pub type MachOComdatIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOComdatIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
