// Generated macro for MachOSymbolTable (struct)
macro_rules! Depcrate_read_macho_symbolMachOSymbolTable {
() => {
// Module: crate::read::macho::symbol
// Provides: {"MachOSymbolTable"}
// Dependencies: {}
# [doc = " A symbol table in a [`MachOFile`]."] # [derive (Debug , Clone , Copy)] pub struct MachOSymbolTable < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , }
};
}
