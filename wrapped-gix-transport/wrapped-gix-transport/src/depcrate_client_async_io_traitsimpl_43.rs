// Generated macro for impl_43 (impl)
macro_rules! Depcrate_client_async_io_traitsimpl_43 {
() => {
// Module: crate::client::async_io::traits
// Provides: {"impl_43"}
// Dependencies: {}
# [async_trait (? Send)] impl < T : Transport + ? Sized > Transport for Box < T > { async fn handshake < 'a > (& mut self , service : Service , extra_parameters : & 'a [(& 'a str , Option < & 'a str >)] ,) -> Result < SetServiceResponse < '_ > , Error > { self . deref_mut () . handshake (service , extra_parameters) . await } fn request (& mut self , write_mode : WriteMode , on_into_read : MessageKind , trace : bool ,) -> Result < RequestWriter < '_ > , Error > { self . deref_mut () . request (write_mode , on_into_read , trace) } }
};
}
