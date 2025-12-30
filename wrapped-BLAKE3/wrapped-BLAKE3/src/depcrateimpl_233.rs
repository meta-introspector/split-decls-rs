// Generated macro for impl_233 (impl)
macro_rules! Depcrateimpl_233 {
() => {
// Module: crate
// Provides: {"impl_233"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl Zeroize for Hasher { fn zeroize (& mut self) { let Self { key , chunk_state , initial_chunk_counter , cv_stack , } = self ; key . zeroize () ; chunk_state . zeroize () ; initial_chunk_counter . zeroize () ; cv_stack . zeroize () ; } }
};
}
