macro_rules! SHA384_OID {
    () => {
        # [cfg (feature = "digest")] const SHA384_OID : ObjectIdentifier = ObjectIdentifier :: new_unwrap ("2.16.840.1.101.3.4.2.2") ;
    };
}

SHA384_OID!();