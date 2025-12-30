// Generated macro for impl_411 (impl)
macro_rules! Depcrate_hirimpl_411 {
() => {
// Module: crate::hir
// Provides: {"impl_411"}
// Dependencies: {}
impl fmt :: Display for Constness { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match * self { Self :: Const => "const" , Self :: NotConst => "non-const" , }) } }
};
}
