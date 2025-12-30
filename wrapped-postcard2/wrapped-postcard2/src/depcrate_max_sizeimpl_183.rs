// Generated macro for impl_183 (impl)
macro_rules! Depcrate_max_sizeimpl_183 {
() => {
// Module: crate::max_size
// Provides: {"impl_183"}
// Dependencies: {}
impl < T : MaxSize > MaxSize for Option < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE + 1 ; }
};
}
