// Generated macro for impl_331 (impl)
macro_rules! Depcrate_hirimpl_331 {
() => {
// Module: crate::hir
// Provides: {"impl_331"}
// Dependencies: {}
impl fmt :: Display for YieldSource { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { YieldSource :: Await { .. } => "`await`" , YieldSource :: Yield => "`yield`" , }) } }
};
}
