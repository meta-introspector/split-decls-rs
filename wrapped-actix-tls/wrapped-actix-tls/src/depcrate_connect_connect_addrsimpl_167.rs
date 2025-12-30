// Generated macro for impl_167 (impl)
macro_rules! Depcrate_connect_connect_addrsimpl_167 {
() => {
// Module: crate::connect::connect_addrs
// Provides: {"impl_167"}
// Dependencies: {}
impl From < Option < SocketAddr > > for ConnectAddrs { fn from (addr : Option < SocketAddr >) -> Self { match addr { Some (addr) => ConnectAddrs :: One (addr) , None => ConnectAddrs :: None , } } }
};
}
