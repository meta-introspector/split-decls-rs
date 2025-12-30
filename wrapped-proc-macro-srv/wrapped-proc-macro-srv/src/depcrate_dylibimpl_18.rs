// Generated macro for impl_18 (impl)
macro_rules! Depcrate_dylibimpl_18 {
() => {
// Module: crate::dylib
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Display for LoadProcMacroDylibError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Io (e) => e . fmt (f) , Self :: AbiMismatch (v) => { use crate :: RUSTC_VERSION_STRING ; write ! (f , "mismatched ABI expected: `{RUSTC_VERSION_STRING}`, got `{v}`") } Self :: LibLoading (e) => e . fmt (f) , } } }
};
}
