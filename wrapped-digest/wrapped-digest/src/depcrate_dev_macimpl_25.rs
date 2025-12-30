// Generated macro for impl_25 (impl)
macro_rules! Depcrate_dev_macimpl_25 {
() => {
// Module: crate::dev::mac
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : OutputSizeUser > Drop for CtOutput < T > { # [inline] fn drop (& mut self) { # [cfg (feature = "zeroize")] { use zeroize :: Zeroize ; self . bytes . zeroize () } } }
};
}
