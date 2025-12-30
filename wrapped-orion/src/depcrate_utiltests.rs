// Generated macro for tests (module)
macro_rules! Depcrate_utiltests {
() => {
// Module: crate::util
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [cfg (feature = "safe_api")] # [test] fn rand_key_len_ok () { let mut dst = [0u8 ; 64] ; secure_rand_bytes (& mut dst) . unwrap () ; } # [cfg (feature = "safe_api")] # [test] fn rand_key_len_error () { let mut dst = [0u8 ; 0] ; assert ! (secure_rand_bytes (& mut dst) . is_err ()) ; let err = secure_rand_bytes (& mut dst) . unwrap_err () ; assert_eq ! (err , errors :: UnknownCryptoError) ; } # [test] fn test_ct_eq_ok () { let buf_1 = [0x06 ; 10] ; let buf_2 = [0x06 ; 10] ; assert ! (secure_cmp (& buf_1 , & buf_2) . is_ok ()) ; assert ! (secure_cmp (& buf_2 , & buf_1) . is_ok ()) ; } # [test] fn test_ct_eq_diff_len () { let buf_1 = [0x06 ; 10] ; let buf_2 = [0x06 ; 5] ; assert ! (secure_cmp (& buf_1 , & buf_2) . is_err ()) ; assert ! (secure_cmp (& buf_2 , & buf_1) . is_err ()) ; } # [test] fn test_ct_ne () { let buf_1 = [0x06 ; 10] ; let buf_2 = [0x76 ; 10] ; assert ! (secure_cmp (& buf_1 , & buf_2) . is_err ()) ; assert ! (secure_cmp (& buf_2 , & buf_1) . is_err ()) ; } # [test] fn test_ct_ne_reg () { assert ! (secure_cmp (& [0] , & [0 , 1]) . is_err ()) ; assert ! (secure_cmp (& [0 , 1] , & [0]) . is_err ()) ; } # [quickcheck] # [cfg (feature = "safe_api")] fn prop_secure_cmp (a : Vec < u8 > , b : Vec < u8 >) -> bool { if a == b { secure_cmp (& a , & b) . is_ok () } else { secure_cmp (& a , & b) . is_err () } } }
};
}
