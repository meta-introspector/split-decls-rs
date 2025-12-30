// Generated macro for OIDS_TO_MIC_NAME (static)
macro_rules! Depcrate_pkcs7OIDS_TO_MIC_NAME {
() => {
// Module: crate::pkcs7
// Provides: {"OIDS_TO_MIC_NAME"}
// Dependencies: {}
static OIDS_TO_MIC_NAME : LazyLock < HashMap < & asn1 :: ObjectIdentifier , & str > > = LazyLock :: new (| | { let mut h = HashMap :: new () ; h . insert (& oid :: SHA224_OID , "sha-224") ; h . insert (& oid :: SHA256_OID , "sha-256") ; h . insert (& oid :: SHA384_OID , "sha-384") ; h . insert (& oid :: SHA512_OID , "sha-512") ; h }) ;
};
}
