// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl client :: ClientSessionStore for ClientCacheWithoutKxHints { fn set_kx_hint (& self , _ : ServerName < 'static > , _ : NamedGroup) { } fn kx_hint (& self , _ : & ServerName < '_ >) -> Option < NamedGroup > { None } fn set_tls12_session (& self , server_name : ServerName < 'static > , mut value : client :: Tls12ClientSessionValue ,) { value . rewind_epoch (self . delay) ; self . storage . set_tls12_session (server_name , value) ; } fn tls12_session (& self , server_name : & ServerName < '_ > ,) -> Option < client :: Tls12ClientSessionValue > { self . storage . tls12_session (server_name) } fn remove_tls12_session (& self , server_name : & ServerName < 'static >) { self . storage . remove_tls12_session (server_name) ; } fn insert_tls13_ticket (& self , server_name : ServerName < 'static > , mut value : client :: Tls13ClientSessionValue ,) { value . rewind_epoch (self . delay) ; self . storage . insert_tls13_ticket (server_name , value) } fn take_tls13_ticket (& self , server_name : & ServerName < 'static > ,) -> Option < client :: Tls13ClientSessionValue > { self . storage . take_tls13_ticket (server_name) } }
};
}
