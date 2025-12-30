// Generated macro for impl_498 (impl)
macro_rules! Depcrate_concurrency_syncimpl_498 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_498"}
// Dependencies: {}
impl fmt :: Display for AccessKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AccessKind :: Read => write ! (f , "read") , AccessKind :: Write => write ! (f , "write") , AccessKind :: Dealloc => write ! (f , "deallocation") , } } }
};
}
