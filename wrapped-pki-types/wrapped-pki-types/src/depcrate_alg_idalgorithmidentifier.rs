// Generated macro for AlgorithmIdentifier (struct)
macro_rules! Depcrate_alg_idAlgorithmIdentifier {
() => {
// Module: crate::alg_id
// Provides: {"AlgorithmIdentifier"}
// Dependencies: {}
# [doc = " A DER encoding of the PKIX AlgorithmIdentifier type:"] # [doc = ""] # [doc = " ```ASN.1"] # [doc = " AlgorithmIdentifier  ::=  SEQUENCE  {"] # [doc = "     algorithm               OBJECT IDENTIFIER,"] # [doc = "     parameters              ANY DEFINED BY algorithm OPTIONAL  }"] # [doc = "                                -- contains a value of the type"] # [doc = "                                -- registered for use with the"] # [doc = "                                -- algorithm object identifier value"] # [doc = " ```"] # [doc = " (from <https://www.rfc-editor.org/rfc/rfc5280#section-4.1.1.2>)"] # [doc = ""] # [doc = " The outer sequence encoding is *not included*, so this is the DER encoding"] # [doc = " of an OID for `algorithm` plus the `parameters` value."] # [doc = ""] # [doc = " For example, this is the `rsaEncryption` algorithm (but prefer to use the constant"] # [doc = " [`RSA_ENCRYPTION`] instead):"] # [doc = ""] # [doc = " ```"] # [doc = " let rsa_encryption = rustls_pki_types::AlgorithmIdentifier::from_slice("] # [doc = "     &["] # [doc = "         // algorithm: 1.2.840.113549.1.1.1"] # [doc = "         0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01,"] # [doc = "         // parameters: NULL"] # [doc = "         0x05, 0x00"] # [doc = "     ]"] # [doc = " );"] # [doc = " assert_eq!(rustls_pki_types::alg_id::RSA_ENCRYPTION, rsa_encryption);"] # [doc = " ```"] # [doc = ""] # [doc = " Common values for this type are provided in this module."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct AlgorithmIdentifier (& 'static [u8]) ;
};
}
