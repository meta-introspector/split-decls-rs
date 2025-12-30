// Generated macro for XcoffSymbolTable64 (type)
macro_rules! Depcrate_read_xcoff_symbolXcoffSymbolTable64 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"XcoffSymbolTable64"}
// Dependencies: {}
# [doc = " A symbol table in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSymbolTable64 < 'data , 'file , R = & 'data [u8] > = XcoffSymbolTable < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
