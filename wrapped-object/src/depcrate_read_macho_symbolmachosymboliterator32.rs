// Generated macro for MachOSymbolIterator32 (type)
macro_rules! Depcrate_read_macho_symbolMachOSymbolIterator32 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbolIterator32"}
// Dependencies: {}
# [doc = " An iterator for the symbols in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSymbolIterator32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbolIterator < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
