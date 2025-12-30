// Generated macro for impl_542 (impl)
macro_rules! Depcrate_vectorimpl_542 {
() => {
// Module: crate::vector
// Provides: {"impl_542"}
// Dependencies: {}
impl < 'a , A : Clone > Iter < 'a , A > { fn new (seq : & 'a Vector < A >) -> Self { Iter { focus : seq . focus () , front_index : 0 , back_index : seq . len () , } } fn from_focus (focus : Focus < 'a , A >) -> Self { Iter { front_index : 0 , back_index : focus . len () , focus , } } }
};
}
