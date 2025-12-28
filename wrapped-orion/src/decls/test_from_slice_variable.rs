macro_rules! test_from_slice_variable {
    () => {
        # [cfg (test)] # [cfg (feature = "safe_api")] macro_rules ! test_from_slice_variable (($ name : ident) => (# [test] # [cfg (feature = "safe_api")] fn test_from_slice_variable () { assert ! ($ name :: from_slice (& [0u8 ; 512]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; 256]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; 1]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; 0]) . is_err ()) ; })) ;
    };
}

test_from_slice_variable!()