// Generated macro for ResolverFut (enum)
macro_rules! Depcrate_connect_resolverResolverFut {
() => {
// Module: crate::connect::resolver
// Provides: {"ResolverFut"}
// Dependencies: {}
# [doc = " Future for resolver service."] # [doc (hidden)] pub enum ResolverFut < R : Host > { Resolved (Option < ConnectInfo < R > >) , LookUp (JoinHandle < io :: Result < IntoIter < SocketAddr > > > , Option < ConnectInfo < R > > ,) , LookupCustom (LocalBoxFuture < 'static , Result < ConnectInfo < R > , ConnectError > >) , }
};
}
