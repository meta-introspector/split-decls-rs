// Generated macro for impl_565 (impl)
macro_rules! Depcrate_vectorimpl_565 {
() => {
// Module: crate::vector
// Provides: {"impl_565"}
// Dependencies: {}
impl < 'a , A : Clone > ChunksMut < 'a , A > { fn new (seq : & 'a mut Vector < A >) -> Self { let len = seq . len () ; ChunksMut { focus : seq . focus_mut () , front_index : 0 , back_index : len , } } }
};
}
