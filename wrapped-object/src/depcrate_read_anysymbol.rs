// Generated macro for Symbol (struct)
macro_rules! Depcrate_read_anySymbol {
() => {
// Module: crate::read::any
// Provides: {"Symbol"}
// Dependencies: {}
# [doc = " An symbol in a [`SymbolTable`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] pub struct Symbol < 'data , 'file , R = & 'data [u8] > where R : ReadRef < 'data > , { inner : SymbolInternal < 'data , 'file , R > , }
};
}
