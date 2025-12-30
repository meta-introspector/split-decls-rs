// Generated macro for SymbolTable (struct)
macro_rules! Depcrate_read_anySymbolTable {
() => {
// Module: crate::read::any
// Provides: {"SymbolTable"}
// Dependencies: {}
# [doc = " A symbol table in a [`File`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbolTable`] trait implementation."] # [derive (Debug)] pub struct SymbolTable < 'data , 'file , R = & 'data [u8] > where R : ReadRef < 'data > , { inner : SymbolTableInternal < 'data , 'file , R > , }
};
}
