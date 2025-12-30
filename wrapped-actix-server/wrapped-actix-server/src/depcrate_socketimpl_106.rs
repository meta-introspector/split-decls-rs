// Generated macro for impl_106 (impl)
macro_rules! Depcrate_socketimpl_106 {
() => {
// Module: crate::socket
// Provides: {"impl_106"}
// Dependencies: {}
impl fmt :: Debug for SocketAddr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Unknown => write ! (f , "Unknown SocketAddr") , Self :: Tcp (ref addr) => write ! (f , "{addr:?}") , # [cfg (unix)] Self :: Uds (ref addr) => write ! (f , "{addr:?}") , } } }
};
}
