// Generated macro for HASH_OIDS_TO_HASH (static)
macro_rules! Depcrate_x509_signHASH_OIDS_TO_HASH {
() => {
// Module: crate::x509::sign
// Provides: {"HASH_OIDS_TO_HASH"}
// Dependencies: {}
static HASH_OIDS_TO_HASH : LazyLock < HashMap < & asn1 :: ObjectIdentifier , & str > > = LazyLock :: new (| | { let mut h = HashMap :: new () ; h . insert (& oid :: SHA1_OID , "SHA1") ; h . insert (& oid :: SHA224_OID , "SHA224") ; h . insert (& oid :: SHA256_OID , "SHA256") ; h . insert (& oid :: SHA384_OID , "SHA384") ; h . insert (& oid :: SHA512_OID , "SHA512") ; h . insert (& oid :: SHA3_224_OID , "SHA3_224") ; h . insert (& oid :: SHA3_256_OID , "SHA3_256") ; h . insert (& oid :: SHA3_384_OID , "SHA3_384") ; h . insert (& oid :: SHA3_512_OID , "SHA3_512") ; h . insert (& oid :: SHA3_224_NIST_OID , "SHA3_224") ; h . insert (& oid :: SHA3_256_NIST_OID , "SHA3_256") ; h . insert (& oid :: SHA3_384_NIST_OID , "SHA3_384") ; h . insert (& oid :: SHA3_512_NIST_OID , "SHA3_512") ; h }) ;
};
}
