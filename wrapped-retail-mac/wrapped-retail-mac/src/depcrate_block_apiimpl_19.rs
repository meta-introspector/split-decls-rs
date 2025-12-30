// Generated macro for impl_19 (impl)
macro_rules! Depcrate_block_apiimpl_19 {
() => {
// Module: crate::block_api
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < C > Drop for RetailMacCore < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone , { fn drop (& mut self) { self . state . zeroize () ; } }
};
}
