// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl Identity { # [doc = " Parses a DER-formatted PKCS #12 archive, using the specified password to decrypt the key."] # [doc = ""] # [doc = " The archive should contain a leaf certificate and its private key, as well any intermediate"] # [doc = " certificates that should be sent to clients to allow them to build a chain to a trusted"] # [doc = " root. The chain certificates should be in order from the leaf certificate towards the root."] # [doc = ""] # [doc = " PKCS #12 archives typically have the file extension `.p12` or `.pfx`, and can be created"] # [doc = " with the OpenSSL `pkcs12` tool:"] # [doc = ""] # [doc = " ```bash"] # [doc = " openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem -certfile chain_certs.pem"] # [doc = " ```"] pub fn from_pkcs12 (der : & [u8] , password : & str) -> Result < Identity > { let identity = imp :: Identity :: from_pkcs12 (der , password) ? ; Ok (Identity (identity)) } # [doc = " Parses a chain of PEM encoded X509 certificates, with the leaf certificate first."] # [doc = " `key` is a PEM encoded PKCS #8 formatted private key for the leaf certificate."] # [doc = ""] # [doc = " The certificate chain should contain any intermediate cerficates that should be sent to"] # [doc = " clients to allow them to build a chain to a trusted root."] # [doc = ""] # [doc = " A certificate chain here means a series of PEM encoded certificates concatenated together."] pub fn from_pkcs8 (pem : & [u8] , key : & [u8]) -> Result < Identity > { let identity = imp :: Identity :: from_pkcs8 (pem , key) ? ; Ok (Identity (identity)) } }
};
}
