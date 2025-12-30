// Generated macro for impl_20 (impl)
macro_rules! Depcrate_block_apiimpl_20 {
() => {
// Module: crate::block_api
// Provides: {"impl_20"}
// Dependencies: {}
impl < C : CmacCipher > Drop for CmacCore < C > { fn drop (& mut self) { # [cfg (feature = "zeroize")] { self . state . zeroize () ; } } }
};
}
