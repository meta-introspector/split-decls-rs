// Generated macro for impl_164 (impl)
macro_rules! Depcrate_hazardous_hash_sha2_sha384impl_164 {
() => {
// Module: crate::hazardous::hash::sha2::sha384
// Provides: {"impl_164"}
// Dependencies: {}
impl Variant < WordU64 , N_CONSTS > for V384 { # [doc = " The SHA384 constants as defined in FIPS 180-4."] const K : [WordU64 ; N_CONSTS] = super :: sha512 :: V512 :: K ; # [rustfmt :: skip] # [allow (clippy :: unreadable_literal)] # [doc = " The SHA384 initial hash value H(0) as defined in FIPS 180-4."] const H0 : [WordU64 ; 8] = [WordU64 (0xcbbb9d5dc1059ed8) , WordU64 (0x629a292a367cd507) , WordU64 (0x9159015a3070dd17) , WordU64 (0x152fecd8f70e5939) , WordU64 (0x67332667ffc00b31) , WordU64 (0x8eb44a8768581511) , WordU64 (0xdb0c2e0d64f98fa7) , WordU64 (0x47b5481dbefa4fa4) ,] ; # [doc = " The Big Sigma 0 function as specified in FIPS 180-4 section 4.1.3."] fn big_sigma_0 (x : WordU64) -> WordU64 { super :: sha512 :: V512 :: big_sigma_0 (x) } # [doc = " The Big Sigma 1 function as specified in FIPS 180-4 section 4.1.3."] fn big_sigma_1 (x : WordU64) -> WordU64 { super :: sha512 :: V512 :: big_sigma_1 (x) } # [doc = " The Small Sigma 0 function as specified in FIPS 180-4 section 4.1.3."] fn small_sigma_0 (x : WordU64) -> WordU64 { super :: sha512 :: V512 :: small_sigma_0 (x) } # [doc = " The Small Sigma 1 function as specified in FIPS 180-4 section 4.1.3."] fn small_sigma_1 (x : WordU64) -> WordU64 { super :: sha512 :: V512 :: small_sigma_1 (x) } }
};
}
