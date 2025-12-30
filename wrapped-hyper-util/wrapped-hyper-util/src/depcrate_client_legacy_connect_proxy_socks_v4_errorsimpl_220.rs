// Generated macro for impl_220 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v4_errorsimpl_220 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v4::errors
// Provides: {"impl_220"}
// Dependencies: {}
impl std :: fmt :: Display for SocksV4Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: IpV6 => f . write_str ("IPV6 is not supported") , Self :: Command (status) => status . fmt (f) , } } }
};
}
