// Generated macro for AuthorityKeyIdentifier (struct)
macro_rules! Depcrate_extensionsAuthorityKeyIdentifier {
() => {
// Module: crate::extensions
// Provides: {"AuthorityKeyIdentifier"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct AuthorityKeyIdentifier < 'a , Op : Asn1Operation > { # [implicit (0)] pub key_identifier : Option < & 'a [u8] > , # [implicit (1)] pub authority_cert_issuer : Option < name :: SequenceOfGeneralName < 'a , Op > > , # [implicit (2)] pub authority_cert_serial_number : Option < SerialNumber < 'a > > , }
};
}
