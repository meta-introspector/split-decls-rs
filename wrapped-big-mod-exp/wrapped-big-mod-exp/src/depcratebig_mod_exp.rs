// Generated macro for big_mod_exp (function)
macro_rules! Depcratebig_mod_exp {
() => {
// Module: crate
// Provides: {"big_mod_exp"}
// Dependencies: {}
# [doc = " Big integer modular exponentiation"] pub fn big_mod_exp (base : & [u8] , exponent : & [u8] , modulus : & [u8]) -> Vec < u8 > { # [cfg (not (target_os = "solana"))] { use { num_bigint :: BigUint , num_traits :: { One , Zero } , } ; let modulus_len = modulus . len () ; let base = BigUint :: from_bytes_be (base) ; let exponent = BigUint :: from_bytes_be (exponent) ; let modulus = BigUint :: from_bytes_be (modulus) ; if modulus . is_zero () || modulus . is_one () { return vec ! [0_u8 ; modulus_len] ; } let ret_int = base . modpow (& exponent , & modulus) ; let ret_int = ret_int . to_bytes_be () ; let mut return_value = vec ! [0_u8 ; modulus_len . saturating_sub (ret_int . len ())] ; return_value . extend (ret_int) ; return_value } # [cfg (target_os = "solana")] { let mut return_value = vec ! [0_u8 ; modulus . len ()] ; let param = BigModExpParams { base : base as * const _ as * const u8 , base_len : base . len () as u64 , exponent : exponent as * const _ as * const u8 , exponent_len : exponent . len () as u64 , modulus : modulus as * const _ as * const u8 , modulus_len : modulus . len () as u64 , } ; unsafe { solana_define_syscall :: definitions :: sol_big_mod_exp (& param as * const _ as * const u8 , return_value . as_mut_slice () as * mut _ as * mut u8 ,) } ; return_value } }
};
}
