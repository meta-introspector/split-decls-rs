// Generated macro for impl_80 (impl)
macro_rules! Depcrate_mpscimpl_80 {
() => {
// Module: crate::mpsc
// Provides: {"impl_80"}
// Dependencies: {}
impl fmt :: Display for TryRecvError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TryRecvError :: Empty => write ! (f , "receive failed because channel is empty") , TryRecvError :: Closed => write ! (f , "receive failed because channel is closed") , } } }
};
}
