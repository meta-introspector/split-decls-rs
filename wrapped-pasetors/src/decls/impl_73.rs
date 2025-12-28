macro_rules! deps {
    () => {
        AsymmetricPublicKey!();
        V2!();
        Id!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        # [cfg (feature = "v2")] impl From < & AsymmetricPublicKey < V2 > > for Id { fn from (key : & AsymmetricPublicKey < V2 >) -> Self { let header = String :: from ("k2.pid.") ; let mut hasher = blake2b :: Blake2b :: new (33) . unwrap () ; hasher . update (header . as_bytes ()) . unwrap () ; let mut paserk_string = String :: new () ; key . fmt (& mut paserk_string) . unwrap () ; hasher . update (paserk_string . as_bytes ()) . unwrap () ; let identifier = encode_b64 (hasher . finalize () . unwrap () . as_ref ()) . unwrap () ; debug_assert_eq ! (identifier . len () , 44) ; Self { header , identifier } } }
    };
}

impl_73!()