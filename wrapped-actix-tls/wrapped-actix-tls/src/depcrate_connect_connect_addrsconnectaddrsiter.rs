// Generated macro for ConnectAddrsIter (enum)
macro_rules! Depcrate_connect_connect_addrsConnectAddrsIter {
() => {
// Module: crate::connect::connect_addrs
// Provides: {"ConnectAddrsIter"}
// Dependencies: {}
# [doc = " Iterator over addresses in a [`Connect`] request."] # [derive (Clone)] pub (crate) enum ConnectAddrsIter < 'a > { None , One (SocketAddr) , Multi (vec_deque :: Iter < 'a , SocketAddr >) , MultiOwned (vec_deque :: IntoIter < SocketAddr >) , }
};
}
