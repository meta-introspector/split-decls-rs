macro_rules! SHA256_OID {
    () => {
        # [cfg (feature = "digest")] const SHA256_OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("2.16.840.1.101.3.4.2.1") ;
    };
}

SHA256_OID!()