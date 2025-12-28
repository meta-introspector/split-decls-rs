macro_rules! deps {
    () => {
        PrimeCurve!();
    };
}

macro_rules! random_transformation_tests {
    () => {
        deps!();
        fn random_transformation_tests < G : PrimeCurve > () { let mut rng = XorShiftRng :: from_seed ([0x59 , 0x62 , 0xbe , 0x5d , 0x76 , 0x3d , 0x31 , 0x8d , 0x17 , 0xdb , 0x37 , 0x32 , 0x54 , 0x06 , 0xbc , 0xe5 ,]) ; for _ in 0 .. 1000 { let g = G :: random (& mut rng) ; let g_affine = g . to_affine () ; let g_projective = g_affine . to_curve () ; assert_eq ! (g , g_projective) ; } for _ in 0 .. 10 { let mut v = (0 .. 1000) . map (| _ | G :: random (& mut rng)) . collect :: < Vec < _ > > () ; use rand :: distributions :: { Distribution , Uniform } ; let between = Uniform :: new (0 , 1000) ; for _ in 0 .. 5 { v [between . sample (& mut rng)] = G :: identity () ; } for _ in 0 .. 5 { let s = between . sample (& mut rng) ; v [s] = v [s] . to_affine () . to_curve () ; } let expected_v = v . iter () . map (| v | v . to_affine ()) . collect :: < Vec < _ > > () ; let mut normalized = vec ! [G :: Affine :: identity () ; v . len ()] ; G :: batch_normalize (& v , & mut normalized) ; assert_eq ! (normalized , expected_v) ; } }
    };
}

random_transformation_tests!();