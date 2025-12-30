// Generated macro for biguint_to_u64_vec (function)
macro_rules! Depcratebiguint_to_u64_vec {
() => {
// Module: crate
// Provides: {"biguint_to_u64_vec"}
// Dependencies: {}
# [doc = " Convert BigUint into a tokenized vector of 64-bit limbs."] fn biguint_to_u64_vec (v : BigUint , limbs : usize) -> proc_macro2 :: TokenStream { let ret = biguint_to_real_u64_vec (v , limbs) ; quote ! ([# (# ret ,) *]) }
};
}
