// Generated macro for impl_61 (impl)
macro_rules! Depcrate_secretimpl_61 {
() => {
// Module: crate::secret
// Provides: {"impl_61"}
// Dependencies: {}
impl Drop for Secret32 { fn drop (& mut self) { self . 0 = [0 ; 32] ; core :: sync :: atomic :: fence (core :: sync :: atomic :: Ordering :: Release) ; } }
};
}
