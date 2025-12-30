// Generated macro for impl_548 (impl)
macro_rules! Depcrate_vectorimpl_548 {
() => {
// Module: crate::vector
// Provides: {"impl_548"}
// Dependencies: {}
impl < 'a , A > IterMut < 'a , A > where A : Clone , { fn new (seq : & 'a mut Vector < A >) -> Self { let focus = seq . focus_mut () ; let len = focus . len () ; IterMut { focus , front_index : 0 , back_index : len , } } fn from_focus (focus : FocusMut < 'a , A >) -> Self { IterMut { front_index : 0 , back_index : focus . len () , focus , } } }
};
}
