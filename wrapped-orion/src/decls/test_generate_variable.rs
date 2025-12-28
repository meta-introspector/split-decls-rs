macro_rules! test_generate_variable {
    () => {
        # [cfg (test)] # [cfg (feature = "safe_api")] macro_rules ! test_generate_variable (($ name : ident) => (# [test] # [cfg (feature = "safe_api")] fn test_generate_variable () { assert ! ($ name :: generate (0) . is_err ()) ; assert ! ($ name :: generate ((isize :: MAX as usize) + 1) . is_err ()) ; assert ! ($ name :: generate (1) . is_ok ()) ; assert ! ($ name :: generate (64) . is_ok ()) ; let test_zero = $ name :: from_slice (& [0u8 ; 128]) . unwrap () ; let test_rand = $ name :: generate (128) . unwrap () ; assert_ne ! (test_zero , test_rand) ; assert_eq ! (test_rand . len () , 128) ; })) ;
    };
}

test_generate_variable!()