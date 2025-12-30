// Generated macro for impl_302 (impl)
macro_rules! Depcrate_hirimpl_302 {
() => {
// Module: crate::hir
// Provides: {"impl_302"}
// Dependencies: {}
impl fmt :: Display for CoroutineSource { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CoroutineSource :: Block => "block" , CoroutineSource :: Closure => "closure body" , CoroutineSource :: Fn => "fn body" , } . fmt (f) } }
};
}
