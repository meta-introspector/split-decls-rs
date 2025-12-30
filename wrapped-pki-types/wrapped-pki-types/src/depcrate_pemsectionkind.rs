// Generated macro for SectionKind (enum)
macro_rules! Depcrate_pemSectionKind {
() => {
// Module: crate::pem
// Provides: {"SectionKind"}
// Dependencies: {}
# [doc = " A single recognised section in a PEM file."] # [non_exhaustive] # [derive (Clone , Copy , Debug , PartialEq)] pub enum SectionKind { # [doc = " A DER-encoded x509 certificate."] # [doc = ""] # [doc = " Appears as \"CERTIFICATE\" in PEM files."] Certificate , # [doc = " A DER-encoded Subject Public Key Info; as specified in RFC 7468."] # [doc = ""] # [doc = " Appears as \"PUBLIC KEY\" in PEM files."] PublicKey , # [doc = " A DER-encoded plaintext RSA private key; as specified in PKCS #1/RFC 3447"] # [doc = ""] # [doc = " Appears as \"RSA PRIVATE KEY\" in PEM files."] RsaPrivateKey , # [doc = " A DER-encoded plaintext private key; as specified in PKCS #8/RFC 5958"] # [doc = ""] # [doc = " Appears as \"PRIVATE KEY\" in PEM files."] PrivateKey , # [doc = " A Sec1-encoded plaintext private key; as specified in RFC 5915"] # [doc = ""] # [doc = " Appears as \"EC PRIVATE KEY\" in PEM files."] EcPrivateKey , # [doc = " A Certificate Revocation List; as specified in RFC 5280"] # [doc = ""] # [doc = " Appears as \"X509 CRL\" in PEM files."] Crl , # [doc = " A Certificate Signing Request; as specified in RFC 2986"] # [doc = ""] # [doc = " Appears as \"CERTIFICATE REQUEST\" in PEM files."] Csr , # [doc = " An EchConfigList structure, as specified in"] # [doc = " <https://www.ietf.org/archive/id/draft-farrell-tls-pemesni-05.html>."] # [doc = ""] # [doc = " Appears as \"ECHCONFIG\" in PEM files."] EchConfigList , }
};
}
