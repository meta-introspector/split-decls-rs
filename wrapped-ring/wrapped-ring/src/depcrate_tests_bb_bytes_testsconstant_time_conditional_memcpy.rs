// Generated macro for constant_time_conditional_memcpy (function)
macro_rules! Depcrate_tests_bb_bytes_testsconstant_time_conditional_memcpy {
() => {
// Module: crate::tests::bb_bytes_tests
// Provides: {"constant_time_conditional_memcpy"}
// Dependencies: {}
# [test] fn constant_time_conditional_memcpy () -> Result < () , error :: Unspecified > { let rng = rand :: SystemRandom :: new () ; for _ in 0 .. 100 { let mut out = rand :: generate :: < [u8 ; 256] > (& rng) ? . expose () ; let input = rand :: generate :: < [u8 ; 256] > (& rng) ? . expose () ; let b = (rand :: generate :: < [u8 ; 1] > (& rng) ? . expose () [0] & 0x0f) == 0 ; let ref_in = input ; let ref_out = if b { input } else { out } ; prefixed_extern ! { fn bssl_constant_time_test_conditional_memcpy (dst : & mut [u8 ; 256] , src : & [u8 ; 256] , b : BoolMask) ; } unsafe { bssl_constant_time_test_conditional_memcpy (& mut out , & input , if b { BoolMask :: TRUE } else { BoolMask :: FALSE } ,) } assert_eq ! (ref_in , input) ; assert_eq ! (ref_out , out) ; } Ok (()) }
};
}
