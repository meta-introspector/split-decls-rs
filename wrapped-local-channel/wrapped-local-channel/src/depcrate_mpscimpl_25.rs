// Generated macro for impl_25 (impl)
macro_rules! Depcrate_mpscimpl_25 {
() => {
// Module: crate::mpsc
// Provides: {"impl_25"}
// Dependencies: {}
impl < T > fmt :: Display for SendError < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "send failed because receiver is gone") } }
};
}
