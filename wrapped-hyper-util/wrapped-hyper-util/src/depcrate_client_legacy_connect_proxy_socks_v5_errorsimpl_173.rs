// Generated macro for impl_173 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_errorsimpl_173 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::errors
// Provides: {"impl_173"}
// Dependencies: {}
impl std :: fmt :: Display for SocksV5Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: HostTooLong => f . write_str ("host address is more than 255 characters") , Self :: Command (e) => e . fmt (f) , Self :: Auth (e) => e . fmt (f) , } } }
};
}
