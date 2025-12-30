// Generated macro for impl_212 (impl)
macro_rules! Depcrate_max_sizeimpl_212 {
() => {
// Module: crate::max_size
// Provides: {"impl_212"}
// Dependencies: {}
impl < A : MaxSize , B : MaxSize , C : MaxSize , D : MaxSize > MaxSize for (A , B , C , D) { const POSTCARD_MAX_SIZE : usize = A :: POSTCARD_MAX_SIZE + B :: POSTCARD_MAX_SIZE + C :: POSTCARD_MAX_SIZE + D :: POSTCARD_MAX_SIZE ; }
};
}
