macro_rules! deps {
    () => {
        SymmetricKey!();
        V2!();
        Id!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        # [cfg (feature = "v2")] impl From < & SymmetricKey < V2 > > for Id { fn from (key : & SymmetricKey < V2 >) -> Self { let header = String :: from ("k2.lid.") ; let mut hasher = blake2b :: Blake2b :: new (33) . unwrap () ; hasher . update (header . as_bytes ()) . unwrap () ; let mut paserk_string = String :: new () ; key . fmt (& mut paserk_string) . unwrap () ; hasher . update (paserk_string . as_bytes ()) . unwrap () ; let identifier = encode_b64 (hasher . finalize () . unwrap () . as_ref ()) . unwrap () ; debug_assert_eq ! (identifier . len () , 44) ; Self { header , identifier } } }
    };
}

impl_69!();