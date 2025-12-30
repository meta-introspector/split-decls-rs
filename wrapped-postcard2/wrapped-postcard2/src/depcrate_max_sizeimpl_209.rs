// Generated macro for impl_209 (impl)
macro_rules! Depcrate_max_sizeimpl_209 {
() => {
// Module: crate::max_size
// Provides: {"impl_209"}
// Dependencies: {}
impl < T : MaxSize > MaxSize for Range < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE * 2 ; }
};
}
