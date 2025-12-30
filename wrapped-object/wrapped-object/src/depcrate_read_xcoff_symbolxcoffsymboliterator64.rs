// Generated macro for XcoffSymbolIterator64 (type)
macro_rules! Depcrate_read_xcoff_symbolXcoffSymbolIterator64 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"XcoffSymbolIterator64"}
// Dependencies: {}
# [doc = " An iterator for the symbols in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSymbolIterator64 < 'data , 'file , R = & 'data [u8] > = XcoffSymbolIterator < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
