// Generated macro for impl_63 (impl)
macro_rules! Depcrate_keysimpl_63 {
() => {
// Module: crate::keys
// Provides: {"impl_63"}
// Dependencies: {}
impl < V > Drop for SymmetricKey < V > { fn drop (& mut self) { use zeroize :: Zeroize ; self . bytes . iter_mut () . zeroize () ; } }
};
}
