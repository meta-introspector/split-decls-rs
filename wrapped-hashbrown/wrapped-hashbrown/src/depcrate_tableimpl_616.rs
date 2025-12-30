// Generated macro for impl_616 (impl)
macro_rules! Depcrate_tableimpl_616 {
() => {
// Module: crate::table
// Provides: {"impl_616"}
// Dependencies: {}
impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Drain < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (Iter { inner : self . inner . iter () , marker : PhantomData , }) . finish () } }
};
}
