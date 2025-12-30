// Generated macro for impl_92 (impl)
macro_rules! Depcrate_client_legacy_connect_dnsimpl_92 {
() => {
// Module: crate::client::legacy::connect::dns
// Provides: {"impl_92"}
// Dependencies: {}
impl Future for GaiFuture { type Output = Result < GaiAddrs , io :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . inner) . poll (cx) . map (| res | match res { Ok (Ok (addrs)) => Ok (GaiAddrs { inner : addrs }) , Ok (Err (err)) => Err (err) , Err (join_err) => { if join_err . is_cancelled () { Err (io :: Error :: new (io :: ErrorKind :: Interrupted , join_err)) } else { panic ! ("gai background task failed: {join_err:?}") } } }) } }
};
}
