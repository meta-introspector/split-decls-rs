// Generated macro for impl_21 (impl)
macro_rules! Depcrate_block_apiimpl_21 {
() => {
// Module: crate::block_api
// Provides: {"impl_21"}
// Dependencies: {}
impl < OS : OutputSize > Drop for BashHashCore < OS > { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . state . zeroize () ; } } }
};
}
