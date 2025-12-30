// Generated macro for random_doubling_tests (function)
macro_rules! Depcrate_testsrandom_doubling_tests {
() => {
// Module: crate::tests
// Provides: {"random_doubling_tests"}
// Dependencies: {}
fn random_doubling_tests < G : PrimeCurve > () { let mut rng = XorShiftRng :: from_seed ([0x59 , 0x62 , 0xbe , 0x5d , 0x76 , 0x3d , 0x31 , 0x8d , 0x17 , 0xdb , 0x37 , 0x32 , 0x54 , 0x06 , 0xbc , 0xe5 ,]) ; for _ in 0 .. 1000 { let mut a = G :: random (& mut rng) ; let mut b = G :: random (& mut rng) ; let tmp1 = (a + b) . double () ; a = a . double () ; b = b . double () ; let mut tmp2 = a ; tmp2 . add_assign (& b) ; let mut tmp3 = a ; tmp3 . add_assign (& b . to_affine ()) ; assert_eq ! (tmp1 , tmp2) ; assert_eq ! (tmp1 , tmp3) ; } }
};
}
