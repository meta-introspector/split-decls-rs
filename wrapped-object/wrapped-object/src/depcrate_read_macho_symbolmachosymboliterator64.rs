// Generated macro for MachOSymbolIterator64 (type)
macro_rules! Depcrate_read_macho_symbolMachOSymbolIterator64 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbolIterator64"}
// Dependencies: {}
# [doc = " An iterator for the symbols in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSymbolIterator64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbolIterator < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
