macro_rules! SHA512_OID {
    () => {
        # [cfg (feature = "digest")] const SHA512_OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("2.16.840.1.101.3.4.2.3") ;
    };
}

SHA512_OID!();