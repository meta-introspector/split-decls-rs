// Generated macro for impl_70 (impl)
macro_rules! Depcrate_mpscimpl_70 {
() => {
// Module: crate::mpsc
// Provides: {"impl_70"}
// Dependencies: {}
impl fmt :: Display for SendError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_full () { write ! (f , "send failed because channel is full") } else { write ! (f , "send failed because receiver is gone") } } }
};
}
