// Generated macro for SignedDataBuilder (struct)
macro_rules! Depcrate_builderSignedDataBuilder {
() => {
// Module: crate::builder
// Provides: {"SignedDataBuilder"}
// Dependencies: {}
# [doc = " Builder for signedData (CMS and PKCS #7)"] pub struct SignedDataBuilder < 's > { digest_algorithms : Vec < AlgorithmIdentifierOwned > , encapsulated_content_info : & 's EncapsulatedContentInfo , certificates : Option < Vec < CertificateChoices > > , crls : Option < Vec < RevocationInfoChoice > > , signer_infos : Vec < SignerInfo > , }
};
}
