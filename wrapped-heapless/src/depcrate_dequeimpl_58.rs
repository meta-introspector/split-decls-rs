// Generated macro for impl_58 (impl)
macro_rules! Depcrate_dequeimpl_58 {
() => {
// Module: crate::deque
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , T : 'a + Copy , S : VecStorage < T > + ? Sized > Extend < & 'a T > for DequeInner < T , S > { fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . copied ()) ; } }
};
}
