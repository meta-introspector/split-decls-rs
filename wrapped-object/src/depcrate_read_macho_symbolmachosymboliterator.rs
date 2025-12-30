// Generated macro for MachOSymbolIterator (struct)
macro_rules! Depcrate_read_macho_symbolMachOSymbolIterator {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbolIterator"}
// Dependencies: {}
# [doc = " An iterator for the symbols in a [`MachOFile`]."] pub struct MachOSymbolIterator < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { file : & 'file MachOFile < 'data , Mach , R > , index : SymbolIndex , }
};
}
