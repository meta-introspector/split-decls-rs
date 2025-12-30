// Generated macro for random_multiplication_tests (function)
macro_rules! Depcrate_testsrandom_multiplication_tests {
() => {
// Module: crate::tests
// Provides: {"random_multiplication_tests"}
// Dependencies: {}
fn random_multiplication_tests < G : PrimeCurve > () { let mut rng = XorShiftRng :: from_seed ([0x59 , 0x62 , 0xbe , 0x5d , 0x76 , 0x3d , 0x31 , 0x8d , 0x17 , 0xdb , 0x37 , 0x32 , 0x54 , 0x06 , 0xbc , 0xe5 ,]) ; for _ in 0 .. 1000 { let mut a = G :: random (& mut rng) ; let mut b = G :: random (& mut rng) ; let a_affine = a . to_affine () ; let b_affine = b . to_affine () ; let s = G :: Scalar :: random (& mut rng) ; let mut tmp1 = a ; tmp1 . add_assign (& b) ; tmp1 . mul_assign (s) ; a . mul_assign (s) ; b . mul_assign (s) ; let mut tmp2 = a ; tmp2 . add_assign (& b) ; let mut tmp3 = Mul :: < G :: Scalar > :: mul (a_affine , s) ; tmp3 . add_assign (Mul :: < G :: Scalar > :: mul (b_affine , s)) ; assert_eq ! (tmp1 , tmp2) ; assert_eq ! (tmp1 , tmp3) ; } }
};
}
