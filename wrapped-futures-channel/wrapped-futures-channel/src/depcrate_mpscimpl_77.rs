// Generated macro for impl_77 (impl)
macro_rules! Depcrate_mpscimpl_77 {
() => {
// Module: crate::mpsc
// Provides: {"impl_77"}
// Dependencies: {}
impl < T > fmt :: Display for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_full () { write ! (f , "send failed because channel is full") } else { write ! (f , "send failed because receiver is gone") } } }
};
}
