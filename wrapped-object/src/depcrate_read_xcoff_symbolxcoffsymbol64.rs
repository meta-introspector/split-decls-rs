// Generated macro for XcoffSymbol64 (type)
macro_rules! Depcrate_read_xcoff_symbolXcoffSymbol64 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"XcoffSymbol64"}
// Dependencies: {}
# [doc = " A symbol in an [`XcoffFile64`](super::XcoffFile64)."] pub type XcoffSymbol64 < 'data , 'file , R = & 'data [u8] > = XcoffSymbol < 'data , 'file , xcoff :: FileHeader64 , R > ;
};
}
