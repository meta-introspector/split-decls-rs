// Generated macro for impl_700 (impl)
macro_rules! Depcrate_iter_foldimpl_700 {
() => {
// Module: crate::iter::fold
// Provides: {"impl_700"}
// Dependencies: {}
impl < I : Debug , U : Debug , F > Debug for FoldWith < I , U , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FoldWith") . field ("base" , & self . base) . field ("item" , & self . item) . finish () } }
};
}
