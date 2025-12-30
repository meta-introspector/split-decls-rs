// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl TlsAcceptorBuilder { # [doc = " Sets the minimum supported protocol version."] # [doc = ""] # [doc = " A value of `None` enables support for the oldest protocols supported by the implementation."] # [doc = ""] # [doc = " Defaults to `Some(Protocol::Tlsv10)`."] pub fn min_protocol_version (& mut self , protocol : Option < Protocol >) -> & mut TlsAcceptorBuilder { self . min_protocol = protocol ; self } # [doc = " Sets the maximum supported protocol version."] # [doc = ""] # [doc = " A value of `None` enables support for the newest protocols supported by the implementation."] # [doc = ""] # [doc = " Defaults to `None`."] pub fn max_protocol_version (& mut self , protocol : Option < Protocol >) -> & mut TlsAcceptorBuilder { self . max_protocol = protocol ; self } # [doc = " Creates a new `TlsAcceptor`."] pub fn build (& self) -> Result < TlsAcceptor > { let acceptor = imp :: TlsAcceptor :: new (self) ? ; Ok (TlsAcceptor (acceptor)) } }
};
}
