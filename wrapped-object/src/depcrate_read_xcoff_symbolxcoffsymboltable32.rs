// Generated macro for XcoffSymbolTable32 (type)
macro_rules! Depcrate_read_xcoff_symbolXcoffSymbolTable32 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"XcoffSymbolTable32"}
// Dependencies: {}
# [doc = " A symbol table in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSymbolTable32 < 'data , 'file , R = & 'data [u8] > = XcoffSymbolTable < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
