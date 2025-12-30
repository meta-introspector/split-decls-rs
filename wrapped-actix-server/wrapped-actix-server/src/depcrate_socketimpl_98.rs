// Generated macro for impl_98 (impl)
macro_rules! Depcrate_socketimpl_98 {
() => {
// Module: crate::socket
// Provides: {"impl_98"}
// Dependencies: {}
impl MioListener { pub (crate) fn local_addr (& self) -> SocketAddr { match * self { MioListener :: Tcp (ref lst) => lst . local_addr () . map (SocketAddr :: Tcp) . unwrap_or (SocketAddr :: Unknown) , # [cfg (unix)] MioListener :: Uds (ref lst) => lst . local_addr () . map (SocketAddr :: Uds) . unwrap_or (SocketAddr :: Unknown) , } } pub (crate) fn accept (& self) -> io :: Result < MioStream > { match * self { MioListener :: Tcp (ref lst) => lst . accept () . map (| (stream , _) | MioStream :: Tcp (stream)) , # [cfg (unix)] MioListener :: Uds (ref lst) => lst . accept () . map (| (stream , _) | MioStream :: Uds (stream)) , } } }
};
}
