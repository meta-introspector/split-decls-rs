// Generated macro for impl_100 (impl)
macro_rules! Depcrate_clientimpl_100 {
() => {
// Module: crate::client
// Provides: {"impl_100"}
// Dependencies: {}
impl ResolvesClientCert for ResolvesClientCertFromChoices { fn resolve (& self , _acceptable_issuers : & [& [u8]] , sig_schemes : & [SignatureScheme] ,) -> Option < Arc < CertifiedKey > > { for key in self . keys . iter () { if key . key . choose_scheme (sig_schemes) . is_some () { return Some (key . clone ()) ; } } None } fn has_certs (& self) -> bool { ! self . keys . is_empty () } }
};
}
