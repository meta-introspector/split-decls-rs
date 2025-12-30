// Generated macro for impl_611 (impl)
macro_rules! Depcrate_tableimpl_611 {
() => {
// Module: crate::table
// Provides: {"impl_611"}
// Dependencies: {}
impl < T , A > fmt :: Debug for IntoIter < T , A > where T : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (Iter { inner : self . inner . iter () , marker : PhantomData , }) . finish () } }
};
}
