// Generated macro for SymbolTable (struct)
macro_rules! Depcrate_read_macho_symbolSymbolTable {
() => {
// Module: crate::read::macho::symbol
// Provides: {"SymbolTable"}
// Dependencies: {}
# [doc = " A table of symbol entries in a Mach-O file."] # [doc = ""] # [doc = " Also includes the string table used for the symbol names."] # [doc = ""] # [doc = " Returned by [`macho::SymtabCommand::symbols`]."] # [derive (Debug , Clone , Copy)] pub struct SymbolTable < 'data , Mach : MachHeader , R = & 'data [u8] > where R : ReadRef < 'data > , { symbols : & 'data [Mach :: Nlist] , strings : StringTable < 'data , R > , }
};
}
