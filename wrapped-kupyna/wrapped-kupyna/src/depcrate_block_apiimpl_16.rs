// Generated macro for impl_16 (impl)
macro_rules! Depcrate_block_apiimpl_16 {
() => {
// Module: crate::block_api
// Provides: {"impl_16"}
// Dependencies: {}
impl Drop for KupynaShortVarCore { # [inline] fn drop (& mut self) { # [cfg (feature = "zeroize")] { self . state . zeroize () ; self . blocks_len . zeroize () ; } } }
};
}
