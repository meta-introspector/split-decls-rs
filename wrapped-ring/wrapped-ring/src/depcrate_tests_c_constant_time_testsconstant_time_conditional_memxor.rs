// Generated macro for constant_time_conditional_memxor (function)
macro_rules! Depcrate_tests_c_constant_time_testsconstant_time_conditional_memxor {
() => {
// Module: crate::tests::c_constant_time_tests
// Provides: {"constant_time_conditional_memxor"}
// Dependencies: {}
# [test] fn constant_time_conditional_memxor () -> Result < () , error :: Unspecified > { let rng = rand :: SystemRandom :: new () ; for _ in 0 .. 256 { let mut out = rand :: generate :: < [u8 ; 256] > (& rng) ? . expose () ; let input = rand :: generate :: < [u8 ; 256] > (& rng) ? . expose () ; let b = (rand :: generate :: < [u8 ; 1] > (& rng) ? . expose () [0] & 0x0f) != 0 ; let ref_in = input ; let mut ref_out = out ; if b { xor_assign_at_start_bytes (& mut ref_out , & ref_in) } ; prefixed_extern ! { fn bssl_constant_time_test_conditional_memxor (dst : & mut [u8 ; 256] , src : & [u8 ; 256] , b : BoolMask) ; } unsafe { bssl_constant_time_test_conditional_memxor (& mut out , & input , if b { BoolMask :: TRUE } else { BoolMask :: FALSE } ,) ; } assert_eq ! (ref_in , input) ; assert_eq ! (ref_out , out) ; } Ok (()) }
};
}
