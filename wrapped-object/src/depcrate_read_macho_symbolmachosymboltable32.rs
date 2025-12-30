// Generated macro for MachOSymbolTable32 (type)
macro_rules! Depcrate_read_macho_symbolMachOSymbolTable32 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbolTable32"}
// Dependencies: {}
# [doc = " A symbol table in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSymbolTable32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbolTable < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
