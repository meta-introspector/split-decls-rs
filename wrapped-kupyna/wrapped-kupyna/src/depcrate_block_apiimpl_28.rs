// Generated macro for impl_28 (impl)
macro_rules! Depcrate_block_apiimpl_28 {
() => {
// Module: crate::block_api
// Provides: {"impl_28"}
// Dependencies: {}
impl Drop for KupynaLongVarCore { # [inline] fn drop (& mut self) { # [cfg (feature = "zeroize")] { self . state . zeroize () ; self . blocks_len . zeroize () ; } } }
};
}
