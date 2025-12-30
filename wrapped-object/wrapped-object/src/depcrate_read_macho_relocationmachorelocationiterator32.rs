// Generated macro for MachORelocationIterator32 (type)
macro_rules! Depcrate_read_macho_relocationMachORelocationIterator32 {
() => {
// Module: crate::read::macho::relocation
// Provides: {"MachORelocationIterator32"}
// Dependencies: {}
# [doc = " An iterator for the relocations in a [`MachOSection32`](super::MachOSection32)."] pub type MachORelocationIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachORelocationIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
