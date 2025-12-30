// Generated macro for impl_210 (impl)
macro_rules! Depcrate_max_sizeimpl_210 {
() => {
// Module: crate::max_size
// Provides: {"impl_210"}
// Dependencies: {}
impl < A : MaxSize , B : MaxSize > MaxSize for (A , B) { const POSTCARD_MAX_SIZE : usize = A :: POSTCARD_MAX_SIZE + B :: POSTCARD_MAX_SIZE ; }
};
}
