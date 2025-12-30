// Generated macro for impl_105 (impl)
macro_rules! Depcrate_socketimpl_105 {
() => {
// Module: crate::socket
// Provides: {"impl_105"}
// Dependencies: {}
impl fmt :: Display for SocketAddr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Self :: Unknown => write ! (f , "Unknown SocketAddr") , Self :: Tcp (ref addr) => write ! (f , "{addr}") , # [cfg (unix)] Self :: Uds (ref addr) => write ! (f , "{addr:?}") , } } }
};
}
