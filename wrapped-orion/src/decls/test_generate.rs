macro_rules! test_generate {
    () => {
        # [cfg (test)] # [cfg (feature = "safe_api")] macro_rules ! test_generate (($ name : ident , $ gen_length : expr) => (# [test] # [cfg (feature = "safe_api")] fn test_generate () { let test_zero = $ name :: from_slice (& [0u8 ; $ gen_length]) . unwrap () ; let test_rand = $ name :: generate () ; assert_ne ! (test_zero , test_rand) ; assert_eq ! (test_rand . len () , $ gen_length) ; })) ;
    };
}

test_generate!()