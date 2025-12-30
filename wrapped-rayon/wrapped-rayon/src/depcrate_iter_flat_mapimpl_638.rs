// Generated macro for impl_638 (impl)
macro_rules! Depcrate_iter_flat_mapimpl_638 {
() => {
// Module: crate::iter::flat_map
// Provides: {"impl_638"}
// Dependencies: {}
impl < I : Debug , F > Debug for FlatMap < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlatMap") . field ("base" , & self . base) . finish () } }
};
}
