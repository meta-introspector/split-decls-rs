// Generated macro for impl_121 (impl)
macro_rules! Depcrate_connectionimpl_121 {
() => {
// Module: crate::connection
// Provides: {"impl_121"}
// Dependencies: {}
impl Connection { pub (crate) fn from_client (conn : ClientConnection) -> Self { Connection { conn : conn . into () , userdata : null_mut () , log_callback : None , } } pub (crate) fn from_server (conn : ServerConnection) -> Self { Connection { conn : conn . into () , userdata : null_mut () , log_callback : None , } } # [allow (dead_code)] pub (crate) fn as_client (& self) -> Option < & ClientConnection > { match & self . conn { rustls :: Connection :: Client (c) => Some (c) , _ => None , } } pub (crate) fn as_server (& self) -> Option < & ServerConnection > { match & self . conn { rustls :: Connection :: Server (s) => Some (s) , _ => None , } } # [allow (dead_code)] pub (crate) fn as_client_mut (& mut self) -> Option < & mut ClientConnection > { match & mut self . conn { rustls :: Connection :: Client (c) => Some (c) , _ => None , } } # [allow (dead_code)] pub (crate) fn as_server_mut (& mut self) -> Option < & mut ServerConnection > { match & mut self . conn { rustls :: Connection :: Server (s) => Some (s) , _ => None , } } }
};
}
