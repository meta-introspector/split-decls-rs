// Generated macro for XcoffSymbolIterator32 (type)
macro_rules! Depcrate_read_xcoff_symbolXcoffSymbolIterator32 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"XcoffSymbolIterator32"}
// Dependencies: {}
# [doc = " An iterator for the symbols in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSymbolIterator32 < 'data , 'file , R = & 'data [u8] > = XcoffSymbolIterator < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
