// Generated macro for MachOSymbolTable64 (type)
macro_rules! Depcrate_read_macho_symbolMachOSymbolTable64 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbolTable64"}
// Dependencies: {}
# [doc = " A symbol table in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSymbolTable64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbolTable < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
