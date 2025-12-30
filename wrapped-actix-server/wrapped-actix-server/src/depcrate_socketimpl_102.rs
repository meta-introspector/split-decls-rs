// Generated macro for impl_102 (impl)
macro_rules! Depcrate_socketimpl_102 {
() => {
// Module: crate::socket
// Provides: {"impl_102"}
// Dependencies: {}
impl fmt :: Debug for MioListener { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { MioListener :: Tcp (ref lst) => write ! (f , "{lst:?}") , # [cfg (unix)] MioListener :: Uds (ref lst) => write ! (f , "{lst:?}") , } } }
};
}
