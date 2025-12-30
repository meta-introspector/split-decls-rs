// Generated macro for impl_sizes (macro)
macro_rules! Depcrate_variantsimpl_sizes {
() => {
// Module: crate::variants
// Provides: {"impl_sizes"}
// Dependencies: {}
macro_rules ! impl_sizes { ($ ($ variant : ident , $ block_size : ident ;) *) => { $ (impl Sealed for typenum ::$ variant { } impl OutputSize for typenum ::$ variant { type BlockSize = typenum ::$ block_size ; }) * } ; }
};
}
