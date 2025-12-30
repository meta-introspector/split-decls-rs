// Generated macro for impl_68 (impl)
macro_rules! Depcrate_keysimpl_68 {
() => {
// Module: crate::keys
// Provides: {"impl_68"}
// Dependencies: {}
impl < V > Drop for AsymmetricSecretKey < V > { fn drop (& mut self) { use zeroize :: Zeroize ; self . bytes . iter_mut () . zeroize () ; } }
};
}
