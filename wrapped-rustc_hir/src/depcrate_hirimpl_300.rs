// Generated macro for impl_300 (impl)
macro_rules! Depcrate_hirimpl_300 {
() => {
// Module: crate::hir
// Provides: {"impl_300"}
// Dependencies: {}
impl fmt :: Display for CoroutineKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CoroutineKind :: Desugared (d , k) => { d . fmt (f) ? ; k . fmt (f) } CoroutineKind :: Coroutine (_) => f . write_str ("coroutine") , } } }
};
}
