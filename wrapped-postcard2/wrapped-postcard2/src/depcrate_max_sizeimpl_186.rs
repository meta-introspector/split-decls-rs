// Generated macro for impl_186 (impl)
macro_rules! Depcrate_max_sizeimpl_186 {
() => {
// Module: crate::max_size
// Provides: {"impl_186"}
// Dependencies: {}
impl < T : MaxSize , const N : usize > MaxSize for [T ; N] { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE * N ; }
};
}
