// Generated macro for SymbolIterator (struct)
macro_rules! Depcrate_read_coff_symbolSymbolIterator {
() => {
// Module: crate::read::coff::symbol
// Provides: {"SymbolIterator"}
// Dependencies: {}
# [doc = " An iterator for symbol entries in a COFF or PE file."] # [doc = ""] # [doc = " Yields the index and symbol structure for each symbol."] # [derive (Debug)] pub struct SymbolIterator < 'data , 'table , R = & 'data [u8] , Coff = pe :: ImageFileHeader > where R : ReadRef < 'data > , Coff : CoffHeader , { symbols : & 'table SymbolTable < 'data , R , Coff > , index : SymbolIndex , }
};
}
