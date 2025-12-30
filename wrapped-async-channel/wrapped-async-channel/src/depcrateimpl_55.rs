// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
impl fmt :: Display for TryRecvError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { TryRecvError :: Empty => write ! (f , "receiving from an empty channel") , TryRecvError :: Closed => write ! (f , "receiving from an empty and closed channel") , } } }
};
}
