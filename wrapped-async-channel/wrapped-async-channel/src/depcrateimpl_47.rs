// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < T > fmt :: Debug for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { TrySendError :: Full (..) => write ! (f , "Full(..)") , TrySendError :: Closed (..) => write ! (f , "Closed(..)") , } } }
};
}
