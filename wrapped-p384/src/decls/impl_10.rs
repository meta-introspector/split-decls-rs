macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [cfg (feature = "pkcs8")] impl pkcs8 :: AssociatedOid for NistP384 { const OID : pkcs8 :: ObjectIdentifier = pkcs8 :: ObjectIdentifier :: new_unwrap ("1.3.132.0.34") ; }
    };
}

impl_10!()