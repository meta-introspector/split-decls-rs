// Generated macro for XcoffSymbolTable (struct)
macro_rules! Depcrate_read_xcoff_symbolXcoffSymbolTable {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"XcoffSymbolTable"}
// Dependencies: {}
# [doc = " A symbol table in an [`XcoffFile`]."] # [derive (Debug , Clone , Copy)] pub struct XcoffSymbolTable < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) symbols : & 'file SymbolTable < 'data , Xcoff , R > , }
};
}
