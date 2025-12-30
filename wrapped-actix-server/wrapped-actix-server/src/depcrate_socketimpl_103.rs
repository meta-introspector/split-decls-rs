// Generated macro for impl_103 (impl)
macro_rules! Depcrate_socketimpl_103 {
() => {
// Module: crate::socket
// Provides: {"impl_103"}
// Dependencies: {}
impl fmt :: Display for MioListener { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { MioListener :: Tcp (ref lst) => write ! (f , "{lst:?}") , # [cfg (unix)] MioListener :: Uds (ref lst) => write ! (f , "{lst:?}") , } } }
};
}
