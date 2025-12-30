// Generated macro for impl_1171 (impl)
macro_rules! Depcrate_read_xcoff_symbolimpl_1171 {
() => {
// Module: crate::read::xcoff::symbol
// Provides: {"impl_1171"}
// Dependencies: {}
impl < 'data , Xcoff , R > Default for SymbolTable < 'data , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { fn default () -> Self { Self { symbols : & [] , strings : StringTable :: default () , header : PhantomData , } } }
};
}
