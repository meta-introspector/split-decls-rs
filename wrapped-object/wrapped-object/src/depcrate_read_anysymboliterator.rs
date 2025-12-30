// Generated macro for SymbolIterator (struct)
macro_rules! Depcrate_read_anySymbolIterator {
() => {
// Module: crate::read::any
// Provides: {"SymbolIterator"}
// Dependencies: {}
# [doc = " An iterator for the symbols in a [`SymbolTable`]."] # [derive (Debug)] pub struct SymbolIterator < 'data , 'file , R = & 'data [u8] > where R : ReadRef < 'data > , { inner : SymbolIteratorInternal < 'data , 'file , R > , }
};
}
