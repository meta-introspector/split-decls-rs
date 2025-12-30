// Generated macro for AuthorityKeyIdentifier (struct)
macro_rules! Depcrate_x509_extensionAuthorityKeyIdentifier {
() => {
// Module: crate::x509::extension
// Provides: {"AuthorityKeyIdentifier"}
// Dependencies: {}
# [doc = " An extension that provides a means of identifying the public key corresponding"] # [doc = " to the private key used to sign a CRL."] pub struct AuthorityKeyIdentifier { critical : bool , keyid : Option < bool > , issuer : Option < bool > , }
};
}
