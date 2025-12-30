// Generated macro for impl_337 (impl)
macro_rules! Depcrate_read_coff_symbolimpl_337 {
() => {
// Module: crate::read::coff::symbol
// Provides: {"impl_337"}
// Dependencies: {}
impl pe :: ImageAuxSymbolWeak { # [doc = " Get the symbol index of the default definition."] pub fn default_symbol (& self) -> SymbolIndex { SymbolIndex (self . weak_default_sym_index . get (LE) as usize) } }
};
}
