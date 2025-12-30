// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < T > fmt :: Display for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { TrySendError :: Full (..) => write ! (f , "sending into a full channel") , TrySendError :: Closed (..) => write ! (f , "sending into a closed channel") , } } }
};
}
