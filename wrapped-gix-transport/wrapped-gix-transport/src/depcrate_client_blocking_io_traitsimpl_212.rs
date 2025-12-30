// Generated macro for impl_212 (impl)
macro_rules! Depcrate_client_blocking_io_traitsimpl_212 {
() => {
// Module: crate::client::blocking_io::traits
// Provides: {"impl_212"}
// Dependencies: {}
impl < T : Transport + ? Sized > Transport for Box < T > { fn handshake < 'a > (& mut self , service : Service , extra_parameters : & 'a [(& 'a str , Option < & 'a str >)] ,) -> Result < SetServiceResponse < '_ > , Error > { self . deref_mut () . handshake (service , extra_parameters) } fn request (& mut self , write_mode : WriteMode , on_into_read : MessageKind , trace : bool ,) -> Result < RequestWriter < '_ > , Error > { self . deref_mut () . request (write_mode , on_into_read , trace) } }
};
}
