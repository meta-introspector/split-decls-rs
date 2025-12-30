// Generated macro for impl_350 (impl)
macro_rules! Depcrate_read_coff_relocationimpl_350 {
() => {
// Module: crate::read::coff::relocation
// Provides: {"impl_350"}
// Dependencies: {}
impl pe :: ImageRelocation { # [doc = " Get the index of the symbol referenced by this relocation."] pub fn symbol (& self) -> SymbolIndex { SymbolIndex (self . symbol_table_index . get (LE) as usize) } }
};
}
