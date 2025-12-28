macro_rules! ECDSA_SHA256_OID {
    () => {
        # [doc = " OID for ECDSA with SHA-256 digests."] # [doc = ""] # [doc = " ```text"] # [doc = " ecdsa-with-SHA256 OBJECT IDENTIFIER ::= { iso(1) member-body(2)"] # [doc = "      us(840) ansi-X9-62(10045) signatures(4) ecdsa-with-SHA2(3) 2 }"] # [doc = " ```"] # [cfg (feature = "digest")] pub const ECDSA_SHA256_OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("1.2.840.10045.4.3.2") ;
    };
}

ECDSA_SHA256_OID!()