// Generated macro for impl_13 (impl)
macro_rules! Depcrate_readimpl_13 {
() => {
// Module: crate::read
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl < BS : ArraySize > Drop for ReadBuffer < BS > { fn drop (& mut self) { use zeroize :: Zeroize ; self . buffer . zeroize () ; } }
};
}
