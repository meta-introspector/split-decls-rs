// Generated macro for impl_192 (impl)
macro_rules! Depcrate_max_sizeimpl_192 {
() => {
// Module: crate::max_size
// Provides: {"impl_192"}
// Dependencies: {}
impl < T : MaxSize , const N : usize > MaxSize for [T ; N] { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE * N ; }
};
}
