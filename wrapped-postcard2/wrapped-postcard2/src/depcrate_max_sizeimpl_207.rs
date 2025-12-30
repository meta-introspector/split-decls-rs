// Generated macro for impl_207 (impl)
macro_rules! Depcrate_max_sizeimpl_207 {
() => {
// Module: crate::max_size
// Provides: {"impl_207"}
// Dependencies: {}
impl < A : MaxSize , B : MaxSize , C : MaxSize , D : MaxSize , E : MaxSize > MaxSize for (A , B , C , D , E) { const POSTCARD_MAX_SIZE : usize = A :: POSTCARD_MAX_SIZE + B :: POSTCARD_MAX_SIZE + C :: POSTCARD_MAX_SIZE + D :: POSTCARD_MAX_SIZE + E :: POSTCARD_MAX_SIZE ; }
};
}
