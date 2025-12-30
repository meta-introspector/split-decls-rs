// Generated macro for ResolvesServerCertFromChoices (struct)
macro_rules! Depcrate_serverResolvesServerCertFromChoices {
() => {
// Module: crate::server
// Provides: {"ResolvesServerCertFromChoices"}
// Dependencies: {}
# [doc = " Choose the server certificate to be used for a connection based on certificate"] # [doc = " type. Will pick the first CertfiedKey available that is suitable for"] # [doc = " the SignatureSchemes supported by the client."] # [derive (Debug)] struct ResolvesServerCertFromChoices { choices : Vec < Arc < CertifiedKey > > , }
};
}
