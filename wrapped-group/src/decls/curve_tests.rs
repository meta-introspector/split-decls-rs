macro_rules! deps {
    () => {
        PrimeCurve!();
    };
}

macro_rules! curve_tests {
    () => {
        deps!();
        pub fn curve_tests < G : PrimeCurve > () { let mut rng = XorShiftRng :: from_seed ([0x59 , 0x62 , 0xbe , 0x5d , 0x76 , 0x3d , 0x31 , 0x8d , 0x17 , 0xdb , 0x37 , 0x32 , 0x54 , 0x06 , 0xbc , 0xe5 ,]) ; { let z = G :: identity () . neg () ; assert ! (bool :: from (z . is_identity ())) ; } { let z = G :: identity () . double () ; assert ! (bool :: from (z . is_identity ())) ; } { let mut r = G :: random (& mut rng) ; let rcopy = r ; r . add_assign (& G :: identity ()) ; assert_eq ! (r , rcopy) ; r . add_assign (& G :: Affine :: identity ()) ; assert_eq ! (r , rcopy) ; let mut z = G :: identity () ; z . add_assign (& G :: identity ()) ; assert ! (bool :: from (z . is_identity ())) ; z . add_assign (& G :: Affine :: identity ()) ; assert ! (bool :: from (z . is_identity ())) ; let mut z2 = z ; z2 . add_assign (& r) ; z . add_assign (& r . to_affine ()) ; assert_eq ! (z , z2) ; assert_eq ! (z , r) ; } { let a = G :: random (& mut rng) ; let b = a . to_affine () . to_curve () ; let c = a . to_affine () . to_curve () . to_affine () . to_curve () ; assert_eq ! (a , b) ; assert_eq ! (b , c) ; } random_addition_tests :: < G > () ; random_multiplication_tests :: < G > () ; random_doubling_tests :: < G > () ; random_negation_tests :: < G > () ; random_transformation_tests :: < G > () ; random_compressed_encoding_tests :: < G > () ; }
    };
}

curve_tests!();