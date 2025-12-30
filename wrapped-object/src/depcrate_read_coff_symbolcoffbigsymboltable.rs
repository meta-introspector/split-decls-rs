// Generated macro for CoffBigSymbolTable (type)
macro_rules! Depcrate_read_coff_symbolCoffBigSymbolTable {
() => {
// Module: crate::read::coff::symbol
// Provides: {"CoffBigSymbolTable"}
// Dependencies: {}
# [doc = " A symbol table in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigSymbolTable < 'data , 'file , R = & 'data [u8] > = CoffSymbolTable < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
};
}
