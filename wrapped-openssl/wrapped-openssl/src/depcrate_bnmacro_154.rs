// Generated macro for macro_154 (macro)
macro_rules! Depcrate_bnmacro_154 {
() => {
// Module: crate::bn
// Provides: {"macro_154"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl , awslc))] { use ffi :: { BN_get_rfc3526_prime_1536 , BN_get_rfc3526_prime_2048 , BN_get_rfc3526_prime_3072 , BN_get_rfc3526_prime_4096 , BN_get_rfc3526_prime_6144 , BN_get_rfc3526_prime_8192 , BN_is_negative , } ; } else if # [cfg (boringssl)] { use ffi :: BN_is_negative ; } else { use ffi :: { get_rfc3526_prime_1536 as BN_get_rfc3526_prime_1536 , get_rfc3526_prime_2048 as BN_get_rfc3526_prime_2048 , get_rfc3526_prime_3072 as BN_get_rfc3526_prime_3072 , get_rfc3526_prime_4096 as BN_get_rfc3526_prime_4096 , get_rfc3526_prime_6144 as BN_get_rfc3526_prime_6144 , get_rfc3526_prime_8192 as BN_get_rfc3526_prime_8192 , } ; # [allow (bad_style)] unsafe fn BN_is_negative (bn : * const ffi :: BIGNUM) -> c_int { (* bn) . neg } } }
};
}
