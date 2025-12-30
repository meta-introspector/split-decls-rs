// Generated macro for impl_19 (impl)
macro_rules! Depcrate_block_apiimpl_19 {
() => {
// Module: crate::block_api
// Provides: {"impl_19"}
// Dependencies: {}
impl Drop for Md4Core { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . state . zeroize () ; self . block_len . zeroize () ; } } }
};
}
