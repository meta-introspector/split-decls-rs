// Generated macro for impl_57 (impl)
macro_rules! Depcrate_dequeimpl_57 {
() => {
// Module: crate::deque
// Provides: {"impl_57"}
// Dependencies: {}
impl < T > fmt :: Debug for Steal < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Empty => f . pad ("Empty") , Self :: Success (_) => f . pad ("Success(..)") , Self :: Retry => f . pad ("Retry") , } } }
};
}
