// Generated macro for impl_215 (impl)
macro_rules! Depcrate_max_sizeimpl_215 {
() => {
// Module: crate::max_size
// Provides: {"impl_215"}
// Dependencies: {}
impl < T : MaxSize > MaxSize for Range < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE * 2 ; }
};
}
