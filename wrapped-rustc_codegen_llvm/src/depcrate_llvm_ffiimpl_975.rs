// Generated macro for impl_975 (impl)
macro_rules! Depcrate_llvm_ffiimpl_975 {
() => {
// Module: crate::llvm::ffi
// Provides: {"impl_975"}
// Dependencies: {}
impl Visibility { pub (crate) fn from_generic (visibility : SymbolVisibility) -> Self { match visibility { SymbolVisibility :: Hidden => Visibility :: Hidden , SymbolVisibility :: Protected => Visibility :: Protected , SymbolVisibility :: Interposable => Visibility :: Default , } } }
};
}
