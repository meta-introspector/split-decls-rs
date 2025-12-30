// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl TlsConnectorBuilder { # [doc = " Sets the identity to be used for client certificate authentication."] pub fn identity (& mut self , _pkcs12 : Pkcs12) -> Result < & mut TlsConnectorBuilder > { loop { } } # [doc = " Sets the protocols which the connector will support."] # [doc = ""] # [doc = " The protocols supported by default are currently TLS 1.0, TLS 1.1, and TLS 1.2, though this"] # [doc = " is subject to change."] pub fn supported_protocols (& mut self , _protocols : & [Protocol] ,) -> Result < & mut TlsConnectorBuilder > { loop { } } # [doc = " Adds a certificate to the set of roots that the connector will trust."] # [doc = ""] # [doc = " The connector will use the system's trust root by default. This method can be used to add"] # [doc = " to that set when communicating with servers not trusted by the system."] pub fn add_root_certificate (& mut self , cert : Certificate) -> Result < & mut TlsConnectorBuilder > { loop { } } # [doc = " Consumes the builder, returning a `TlsConnector`."] pub fn build (self) -> Result < TlsConnector > { loop { } } }
};
}
