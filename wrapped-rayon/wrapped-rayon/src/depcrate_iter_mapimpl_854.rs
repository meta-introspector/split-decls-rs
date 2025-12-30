// Generated macro for impl_854 (impl)
macro_rules! Depcrate_iter_mapimpl_854 {
() => {
// Module: crate::iter::map
// Provides: {"impl_854"}
// Dependencies: {}
impl < I : Debug , F > Debug for Map < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Map") . field ("base" , & self . base) . finish () } }
};
}
