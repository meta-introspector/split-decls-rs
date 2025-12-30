// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl Pkcs12 { # [doc = " Parses a DER-formatted PKCS #12 archive, using the specified password to decrypt the key."] # [doc = ""] # [doc = " The archive should contain a leaf certificate and its private key, as well any intermediate"] # [doc = " certificates that should be sent to clients to allow them to build a chain to a trusted"] # [doc = " root. The chain certificates should be in order from the leaf certificate towards the root."] # [doc = ""] # [doc = " PKCS #12 archives typically have the file extension `.p12` or `.pfx`, and can be created"] # [doc = " with the OpenSSL `pkcs12` tool:"] # [doc = ""] # [doc = " ```bash"] # [doc = " openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem -certfile chain_certs.pem"] # [doc = " ```"] pub fn from_der (_der : & [u8] , _password : & str) -> Result < Pkcs12 > { unimplemented ! () } }
};
}
