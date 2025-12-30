// Generated macro for CoffBigSymbol (type)
macro_rules! Depcrate_read_coff_symbolCoffBigSymbol {
() => {
// Module: crate::read::coff::symbol
// Provides: {"CoffBigSymbol"}
// Dependencies: {}
# [doc = " A symbol in a [`CoffBigFile`](super::CoffBigFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] pub type CoffBigSymbol < 'data , 'file , R = & 'data [u8] > = CoffSymbol < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
