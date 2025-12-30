// Generated macro for impl_605 (impl)
macro_rules! Depcrate_tableimpl_605 {
() => {
// Module: crate::table
// Provides: {"impl_605"}
// Dependencies: {}
impl < T > fmt :: Debug for IterHashMut < '_ , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (IterHash { inner : self . inner . clone () , marker : PhantomData , }) . finish () } }
};
}
