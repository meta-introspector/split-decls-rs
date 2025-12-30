// Generated macro for SignerInfo (struct)
macro_rules! Depcrate_pkcs7SignerInfo {
() => {
// Module: crate::pkcs7
// Provides: {"SignerInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct SignerInfo < 'a > { pub version : u8 , pub issuer_and_serial_number : IssuerAndSerialNumber < 'a > , pub digest_algorithm : common :: AlgorithmIdentifier < 'a > , # [implicit (0)] pub authenticated_attributes : Option < csr :: Attributes < 'a > > , pub digest_encryption_algorithm : common :: AlgorithmIdentifier < 'a > , pub encrypted_digest : & 'a [u8] , # [implicit (1)] pub unauthenticated_attributes : Option < csr :: Attributes < 'a > > , }
};
}
