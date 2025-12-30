// Generated macro for impl_233 (impl)
macro_rules! Depcrate_connect_resolverimpl_233 {
() => {
// Module: crate::connect::resolver
// Provides: {"impl_233"}
// Dependencies: {}
impl ResolverService { # [doc = " Constructor for custom Resolve trait object and use it as resolver."] pub fn custom (resolver : impl Resolve + 'static) -> Self { Self { kind : ResolverKind :: Custom (Rc :: new (resolver)) , } } # [doc = " Resolve DNS with default resolver."] fn default_lookup < R : Host > (req : & ConnectInfo < R > ,) -> JoinHandle < io :: Result < IntoIter < SocketAddr > > > { let host = format ! ("{}:{}" , req . hostname () , req . port ()) ; spawn_blocking (move | | std :: net :: ToSocketAddrs :: to_socket_addrs (& host)) } }
};
}
