macro_rules! deps {
    () => {
        PrimeCurve!();
    };
}

macro_rules! random_negation_tests {
    () => {
        deps!();
        fn random_negation_tests < G : PrimeCurve > () { let mut rng = XorShiftRng :: from_seed ([0x59 , 0x62 , 0xbe , 0x5d , 0x76 , 0x3d , 0x31 , 0x8d , 0x17 , 0xdb , 0x37 , 0x32 , 0x54 , 0x06 , 0xbc , 0xe5 ,]) ; for _ in 0 .. 1000 { let r = G :: random (& mut rng) ; let s = G :: Scalar :: random (& mut rng) ; let sneg = s . neg () ; let mut t1 = r ; t1 . mul_assign (s) ; let mut t2 = r ; t2 . mul_assign (sneg) ; let mut t3 = t1 ; t3 . add_assign (& t2) ; assert ! (bool :: from (t3 . is_identity ())) ; let mut t4 = t1 ; t4 . add_assign (& t2 . to_affine ()) ; assert ! (bool :: from (t4 . is_identity ())) ; assert_eq ! (t1 . neg () , t2) ; } }
    };
}

random_negation_tests!()