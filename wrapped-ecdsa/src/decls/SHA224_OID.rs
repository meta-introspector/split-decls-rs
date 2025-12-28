macro_rules! SHA224_OID {
    () => {
        # [cfg (feature = "digest")] const SHA224_OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("2.16.840.1.101.3.4.2.4") ;
    };
}

SHA224_OID!();