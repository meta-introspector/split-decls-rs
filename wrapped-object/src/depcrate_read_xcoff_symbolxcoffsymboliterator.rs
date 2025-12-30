// Generated macro for XcoffSymbolIterator (struct)
macro_rules! Depcrate_read_xcoff_symbolXcoffSymbolIterator {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"XcoffSymbolIterator"}
// Dependencies: {}
# [doc = " An iterator for the symbols in an [`XcoffFile`]."] pub struct XcoffSymbolIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , pub (super) symbols : SymbolIterator < 'data , 'file , Xcoff , R > , }
};
}
