// Generated macro for ContentInfo (struct)
macro_rules! Depcrate_pkcs7ContentInfo {
() => {
// Module: crate::pkcs7
// Provides: {"ContentInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write , asn1 :: Asn1Read)] pub struct ContentInfo < 'a > { pub _content_type : asn1 :: DefinedByMarker < asn1 :: ObjectIdentifier > , # [defined_by (_content_type)] pub content : Content < 'a > , }
};
}
