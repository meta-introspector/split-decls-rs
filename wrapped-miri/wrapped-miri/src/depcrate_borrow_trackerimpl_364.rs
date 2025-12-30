// Generated macro for impl_364 (impl)
macro_rules! Depcrate_borrow_trackerimpl_364 {
() => {
// Module: crate::borrow_tracker
// Provides: {"impl_364"}
// Dependencies: {}
impl fmt :: Display for AccessKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AccessKind :: Read => write ! (f , "read access") , AccessKind :: Write => write ! (f , "write access") , } } }
};
}
