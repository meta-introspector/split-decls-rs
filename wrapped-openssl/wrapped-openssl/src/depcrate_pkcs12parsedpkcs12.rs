// Generated macro for ParsedPkcs12 (struct)
macro_rules! Depcrate_pkcs12ParsedPkcs12 {
() => {
// Module: crate::pkcs12
// Provides: {"ParsedPkcs12"}
// Dependencies: {}
# [deprecated (note = "Use ParsedPkcs12_2 instead" , since = "0.10.46")] pub struct ParsedPkcs12 { pub pkey : PKey < Private > , pub cert : X509 , pub chain : Option < Stack < X509 > > , }
};
}
