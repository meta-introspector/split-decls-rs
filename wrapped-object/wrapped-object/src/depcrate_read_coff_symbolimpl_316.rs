// Generated macro for impl_316 (impl)
macro_rules! Depcrate_read_coff_symbolimpl_316 {
() => {
// Module: crate::read::coff::symbol
// Provides: {"impl_316"}
// Dependencies: {}
impl < 'data , R : ReadRef < 'data > , Coff : CoffHeader > Default for SymbolTable < 'data , R , Coff > { fn default () -> Self { Self { symbols : & [] , strings : StringTable :: default () , } } }
};
}
