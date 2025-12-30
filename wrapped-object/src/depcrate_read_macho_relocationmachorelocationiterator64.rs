// Generated macro for MachORelocationIterator64 (type)
macro_rules! Depcrate_read_macho_relocationMachORelocationIterator64 {
() => {
// Module: crate::read::macho::relocation
// Provides: {"MachORelocationIterator64"}
// Dependencies: {}
# [doc = " An iterator for the relocations in a [`MachOSection64`](super::MachOSection64)."] pub type MachORelocationIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachORelocationIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
