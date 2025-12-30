// Generated macro for impl_30 (impl)
macro_rules! Depcrate_block_apiimpl_30 {
() => {
// Module: crate::block_api
// Provides: {"impl_30"}
// Dependencies: {}
impl < P : Gost94Params > Drop for Gost94Core < P > { fn drop (& mut self) { # [cfg (feature = "zeroize")] { self . h . zeroize () ; self . n . zeroize () ; self . sigma . zeroize () ; } } }
};
}
