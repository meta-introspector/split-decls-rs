// Generated macro for impl_691 (impl)
macro_rules! Depcrate_iter_foldimpl_691 {
() => {
// Module: crate::iter::fold
// Provides: {"impl_691"}
// Dependencies: {}
impl < I : Debug , ID , F > Debug for Fold < I , ID , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Fold") . field ("base" , & self . base) . finish () } }
};
}
