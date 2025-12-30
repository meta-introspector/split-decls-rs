// Generated macro for impl_279 (impl)
macro_rules! Depcrate_hirimpl_279 {
() => {
// Module: crate::hir
// Provides: {"impl_279"}
// Dependencies: {}
impl fmt :: Display for RangeEnd { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { RangeEnd :: Included => "..=" , RangeEnd :: Excluded => ".." , }) } }
};
}
