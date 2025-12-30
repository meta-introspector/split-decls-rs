// Generated macro for XcoffSymbol32 (type)
macro_rules! Depcrate_read_xcoff_symbolXcoffSymbol32 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"XcoffSymbol32"}
// Dependencies: {}
# [doc = " A symbol in an [`XcoffFile32`](super::XcoffFile32)."] pub type XcoffSymbol32 < 'data , 'file , R = & 'data [u8] > = XcoffSymbol < 'data , 'file , xcoff :: FileHeader32 , R > ;
};
}
