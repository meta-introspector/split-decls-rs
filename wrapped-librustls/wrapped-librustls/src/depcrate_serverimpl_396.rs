// Generated macro for impl_396 (impl)
macro_rules! Depcrate_serverimpl_396 {
() => {
// Module: crate::server
// Provides: {"impl_396"}
// Dependencies: {}
impl ResolvesServerCert for ResolvesServerCertFromChoices { fn resolve (& self , client_hello : ClientHello) -> Option < Arc < CertifiedKey > > { for key in self . choices . iter () { if key . key . choose_scheme (client_hello . signature_schemes ()) . is_some () { return Some (key . clone ()) ; } } None } }
};
}
