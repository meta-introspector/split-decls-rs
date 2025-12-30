// Generated macro for impl_236 (impl)
macro_rules! Depcrate_connect_resolverimpl_236 {
() => {
// Module: crate::connect::resolver
// Provides: {"impl_236"}
// Dependencies: {}
impl < R : Host > Future for ResolverFut < R > { type Output = Result < ConnectInfo < R > , ConnectError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . get_mut () { Self :: Resolved (conn) => Poll :: Ready (Ok (conn . take () . expect ("ResolverFuture polled after finished"))) , Self :: LookUp (fut , req) => { let res = match ready ! (Pin :: new (fut) . poll (cx)) { Ok (Ok (res)) => Ok (res) , Ok (Err (err)) => Err (ConnectError :: Resolver (Box :: new (err))) , Err (err) => Err (ConnectError :: Io (err . into ())) , } ; let req = req . take () . unwrap () ; let addrs = res . map_err (| err | { trace ! ("DNS resolver: failed to resolve host {:?} err: {:?}" , req . hostname () , err) ; err }) ? ; let req = req . set_addrs (addrs) ; trace ! ("DNS resolver: host {:?} resolved to {:?}" , req . hostname () , req . addrs ()) ; if req . addr . is_unresolved () { Poll :: Ready (Err (ConnectError :: NoRecords)) } else { Poll :: Ready (Ok (req)) } } Self :: LookupCustom (fut) => fut . as_mut () . poll (cx) , } } }
};
}
