// Generated macro for impl_213 (impl)
macro_rules! Depcrate_max_sizeimpl_213 {
() => {
// Module: crate::max_size
// Provides: {"impl_213"}
// Dependencies: {}
impl < A : MaxSize , B : MaxSize , C : MaxSize , D : MaxSize , E : MaxSize > MaxSize for (A , B , C , D , E) { const POSTCARD_MAX_SIZE : usize = A :: POSTCARD_MAX_SIZE + B :: POSTCARD_MAX_SIZE + C :: POSTCARD_MAX_SIZE + D :: POSTCARD_MAX_SIZE + E :: POSTCARD_MAX_SIZE ; }
};
}
