macro_rules! deps {
    () => {
        PrimeCurve!();
    };
}

macro_rules! random_compressed_encoding_tests {
    () => {
        deps!();
        fn random_compressed_encoding_tests < G : PrimeCurve > () { let mut rng = XorShiftRng :: from_seed ([0x59 , 0x62 , 0xbe , 0x5d , 0x76 , 0x3d , 0x31 , 0x8d , 0x17 , 0xdb , 0x37 , 0x32 , 0x54 , 0x06 , 0xbc , 0xe5 ,]) ; assert_eq ! (G :: Affine :: from_bytes (& G :: Affine :: identity () . to_bytes ()) . unwrap () , G :: Affine :: identity ()) ; for _ in 0 .. 1000 { let mut r = G :: random (& mut rng) . to_affine () ; let compressed = r . to_bytes () ; let de_compressed = G :: Affine :: from_bytes (& compressed) . unwrap () ; assert_eq ! (de_compressed , r) ; r = r . neg () ; let compressed = r . to_bytes () ; let de_compressed = G :: Affine :: from_bytes (& compressed) . unwrap () ; assert_eq ! (de_compressed , r) ; } }
    };
}

random_compressed_encoding_tests!()