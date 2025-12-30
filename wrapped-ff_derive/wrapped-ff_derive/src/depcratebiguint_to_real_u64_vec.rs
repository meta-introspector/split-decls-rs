// Generated macro for biguint_to_real_u64_vec (function)
macro_rules! Depcratebiguint_to_real_u64_vec {
() => {
// Module: crate
// Provides: {"biguint_to_real_u64_vec"}
// Dependencies: {}
# [doc = " Convert BigUint into a vector of 64-bit limbs."] fn biguint_to_real_u64_vec (mut v : BigUint , limbs : usize) -> Vec < u64 > { let m = BigUint :: one () << 64 ; let mut ret = vec ! [] ; while v > BigUint :: zero () { let limb : BigUint = & v % & m ; ret . push (limb . to_u64 () . unwrap ()) ; v >>= 64 ; } while ret . len () < limbs { ret . push (0) ; } assert ! (ret . len () == limbs) ; ret }
};
}
