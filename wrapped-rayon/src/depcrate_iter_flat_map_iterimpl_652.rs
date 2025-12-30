// Generated macro for impl_652 (impl)
macro_rules! Depcrate_iter_flat_map_iterimpl_652 {
() => {
// Module: crate::iter::flat_map_iter
// Provides: {"impl_652"}
// Dependencies: {}
impl < I : Debug , F > Debug for FlatMapIter < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlatMapIter") . field ("base" , & self . base) . finish () } }
};
}
