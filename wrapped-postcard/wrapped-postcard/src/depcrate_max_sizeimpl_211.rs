// Generated macro for impl_211 (impl)
macro_rules! Depcrate_max_sizeimpl_211 {
() => {
// Module: crate::max_size
// Provides: {"impl_211"}
// Dependencies: {}
impl < A : MaxSize , B : MaxSize , C : MaxSize > MaxSize for (A , B , C) { const POSTCARD_MAX_SIZE : usize = A :: POSTCARD_MAX_SIZE + B :: POSTCARD_MAX_SIZE + C :: POSTCARD_MAX_SIZE ; }
};
}
