// Generated macro for MachOSymbol64 (type)
macro_rules! Depcrate_read_macho_symbolMachOSymbol64 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbol64"}
// Dependencies: {}
# [doc = " A symbol in a [`MachOFile64`](super::MachOFile64)."] pub type MachOSymbol64 < 'data , 'file , Endian = Endianness , R = & 'data [u8] > = MachOSymbol < 'data , 'file , macho :: MachHeader64 < Endian > , R > ;
};
}
