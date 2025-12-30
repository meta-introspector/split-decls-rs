// Generated macro for test_generate (macro)
macro_rules! Depcrate_typedefstest_generate {
() => {
// Module: crate::typedefs
// Provides: {"test_generate"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "safe_api")] macro_rules ! test_generate (($ name : ident , $ gen_length : expr) => (# [test] # [cfg (feature = "safe_api")] fn test_generate () { let test_zero = $ name :: from_slice (& [0u8 ; $ gen_length]) . unwrap () ; let test_rand = $ name :: generate () ; assert_ne ! (test_zero , test_rand) ; assert_eq ! (test_rand . len () , $ gen_length) ; })) ;
};
}
