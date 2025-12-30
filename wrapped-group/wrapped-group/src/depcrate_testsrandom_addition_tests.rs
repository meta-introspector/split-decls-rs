// Generated macro for random_addition_tests (function)
macro_rules! Depcrate_testsrandom_addition_tests {
() => {
// Module: crate::tests
// Provides: {"random_addition_tests"}
// Dependencies: {}
fn random_addition_tests < G : PrimeCurve > () { let mut rng = XorShiftRng :: from_seed ([0x59 , 0x62 , 0xbe , 0x5d , 0x76 , 0x3d , 0x31 , 0x8d , 0x17 , 0xdb , 0x37 , 0x32 , 0x54 , 0x06 , 0xbc , 0xe5 ,]) ; for _ in 0 .. 1000 { let a = G :: random (& mut rng) ; let b = G :: random (& mut rng) ; let c = G :: random (& mut rng) ; let a_affine = a . to_affine () ; let b_affine = b . to_affine () ; let c_affine = c . to_affine () ; { let mut aplusa = a ; aplusa . add_assign (& a) ; let mut aplusamixed = a ; aplusamixed . add_assign (& a . to_affine ()) ; let adouble = a . double () ; assert_eq ! (aplusa , adouble) ; assert_eq ! (aplusa , aplusamixed) ; } let mut tmp = vec ! [G :: identity () ; 6] ; tmp [0] = a ; tmp [0] . add_assign (& b) ; tmp [0] . add_assign (& c) ; tmp [1] = b ; tmp [1] . add_assign (& c) ; tmp [1] . add_assign (& a) ; tmp [2] = a ; tmp [2] . add_assign (& c) ; tmp [2] . add_assign (& b) ; tmp [3] = a_affine . to_curve () ; tmp [3] . add_assign (& b_affine) ; tmp [3] . add_assign (& c_affine) ; tmp [4] = b_affine . to_curve () ; tmp [4] . add_assign (& c_affine) ; tmp [4] . add_assign (& a_affine) ; tmp [5] = a_affine . to_curve () ; tmp [5] . add_assign (& c_affine) ; tmp [5] . add_assign (& b_affine) ; for i in 0 .. 6 { for j in 0 .. 6 { assert_eq ! (tmp [i] , tmp [j]) ; assert_eq ! (tmp [i] . to_affine () , tmp [j] . to_affine ()) ; } assert ! (tmp [i] != a) ; assert ! (tmp [i] != b) ; assert ! (tmp [i] != c) ; assert ! (a != tmp [i]) ; assert ! (b != tmp [i]) ; assert ! (c != tmp [i]) ; } } }
};
}
