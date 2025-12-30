// Generated macro for impl_208 (impl)
macro_rules! Depcrate_max_sizeimpl_208 {
() => {
// Module: crate::max_size
// Provides: {"impl_208"}
// Dependencies: {}
impl < A : MaxSize , B : MaxSize , C : MaxSize , D : MaxSize , E : MaxSize , F : MaxSize > MaxSize for (A , B , C , D , E , F) { const POSTCARD_MAX_SIZE : usize = A :: POSTCARD_MAX_SIZE + B :: POSTCARD_MAX_SIZE + C :: POSTCARD_MAX_SIZE + D :: POSTCARD_MAX_SIZE + E :: POSTCARD_MAX_SIZE + F :: POSTCARD_MAX_SIZE ; }
};
}
