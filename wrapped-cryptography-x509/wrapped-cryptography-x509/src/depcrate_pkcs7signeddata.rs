// Generated macro for SignedData (struct)
macro_rules! Depcrate_pkcs7SignedData {
() => {
// Module: crate::pkcs7
// Provides: {"SignedData"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct SignedData < 'a > { pub version : u8 , pub digest_algorithms : common :: Asn1ReadableOrWritable < asn1 :: SetOf < 'a , common :: AlgorithmIdentifier < 'a > > , asn1 :: SetOfWriter < 'a , common :: AlgorithmIdentifier < 'a > > , > , pub content_info : ContentInfo < 'a > , # [implicit (0)] pub certificates : Option < common :: Asn1ReadableOrWritable < asn1 :: SetOf < 'a , certificate :: Certificate < 'a > > , asn1 :: SetOfWriter < 'a , certificate :: Certificate < 'a > > , > , > , # [implicit (1)] pub crls : Option < common :: Asn1ReadableOrWritable < asn1 :: SetOf < 'a , asn1 :: Sequence < 'a > > , asn1 :: SetOfWriter < 'a , asn1 :: Sequence < 'a > > , > , > , pub signer_infos : common :: Asn1ReadableOrWritable < asn1 :: SetOf < 'a , SignerInfo < 'a > > , asn1 :: SetOfWriter < 'a , SignerInfo < 'a > > , > , }
};
}
