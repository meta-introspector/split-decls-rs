// Generated macro for impl_57 (impl)
macro_rules! Depcrate_secretimpl_57 {
() => {
// Module: crate::secret
// Provides: {"impl_57"}
// Dependencies: {}
impl Drop for Secret16 { fn drop (& mut self) { self . 0 = [0 ; 16] ; core :: sync :: atomic :: fence (core :: sync :: atomic :: Ordering :: Release) ; } }
};
}
