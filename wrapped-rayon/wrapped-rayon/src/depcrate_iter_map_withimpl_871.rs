// Generated macro for impl_871 (impl)
macro_rules! Depcrate_iter_map_withimpl_871 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_871"}
// Dependencies: {}
impl < I : Debug , T : Debug , F > Debug for MapWith < I , T , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MapWith") . field ("base" , & self . base) . field ("item" , & self . item) . finish () } }
};
}
