// Generated macro for impl_174 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_socks_v5_errorsimpl_174 {
() => {
// Module: crate::client::legacy::connect::proxy::socks::v5::errors
// Provides: {"impl_174"}
// Dependencies: {}
impl std :: fmt :: Display for AuthError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { Self :: Unsupported => "server does not support user/pass authentication" , Self :: MethodMismatch => "server implements authentication incorrectly" , Self :: Failed => "credentials not accepted" , }) } }
};
}
