// Generated macro for impl_189 (impl)
macro_rules! Depcrate_max_sizeimpl_189 {
() => {
// Module: crate::max_size
// Provides: {"impl_189"}
// Dependencies: {}
impl < T : MaxSize > MaxSize for Option < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE + 1 ; }
};
}
