// Generated macro for impl_19 (impl)
macro_rules! Depcrate_block_apiimpl_19 {
() => {
// Module: crate::block_api
// Provides: {"impl_19"}
// Dependencies: {}
impl Drop for BeltHashCore { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . r . zeroize () ; self . s . zeroize () ; self . h . zeroize () ; } } }
};
}
