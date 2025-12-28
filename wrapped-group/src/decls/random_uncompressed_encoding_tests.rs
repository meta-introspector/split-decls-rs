macro_rules! deps {
    () => {
        UncompressedEncoding!();
        PrimeCurve!();
    };
}

macro_rules! random_uncompressed_encoding_tests {
    () => {
        deps!();
        pub fn random_uncompressed_encoding_tests < G : PrimeCurve > () where < G as PrimeCurve > :: Affine : UncompressedEncoding , { let mut rng = XorShiftRng :: from_seed ([0x59 , 0x62 , 0xbe , 0x5d , 0x76 , 0x3d , 0x31 , 0x8d , 0x17 , 0xdb , 0x37 , 0x32 , 0x54 , 0x06 , 0xbc , 0xe5 ,]) ; assert_eq ! (G :: Affine :: from_uncompressed (& G :: Affine :: identity () . to_uncompressed ()) . unwrap () , G :: Affine :: identity ()) ; for _ in 0 .. 1000 { let r = G :: random (& mut rng) . to_affine () ; let uncompressed = r . to_uncompressed () ; let de_uncompressed = G :: Affine :: from_uncompressed (& uncompressed) . unwrap () ; assert_eq ! (de_uncompressed , r) ; } }
    };
}

random_uncompressed_encoding_tests!()