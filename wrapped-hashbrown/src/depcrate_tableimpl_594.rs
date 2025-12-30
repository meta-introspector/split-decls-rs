// Generated macro for impl_594 (impl)
macro_rules! Depcrate_tableimpl_594 {
() => {
// Module: crate::table
// Provides: {"impl_594"}
// Dependencies: {}
impl < T > fmt :: Debug for IterMut < '_ , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (Iter { inner : self . inner . clone () , marker : PhantomData , }) . finish () } }
};
}
