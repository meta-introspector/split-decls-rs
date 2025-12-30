// Generated macro for CoffBigSymbolIterator (type)
macro_rules! Depcrate_read_coff_symbolCoffBigSymbolIterator {
() => {
// Module: crate::read::coff::symbol
// Provides: {"CoffBigSymbolIterator"}
// Dependencies: {}
# [doc = " An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigSymbolIterator < 'data , 'file , R = & 'data [u8] > = CoffSymbolIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
