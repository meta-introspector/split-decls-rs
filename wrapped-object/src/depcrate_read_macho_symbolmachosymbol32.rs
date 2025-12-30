// Generated macro for MachOSymbol32 (type)
macro_rules! Depcrate_read_macho_symbolMachOSymbol32 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbol32"}
// Dependencies: {}
# [doc = " A symbol in a [`MachOFile32`](super::MachOFile32)."] pub type MachOSymbol32 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbol < 'data , 'file , macho :: MachHeader32 < Endian > , R > ;
};
}
