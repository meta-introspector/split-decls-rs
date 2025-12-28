macro_rules! test_normal_debug {
    () => {
        # [cfg (test)] # [cfg (feature = "safe_api")] macro_rules ! test_normal_debug (($ name : ident , $ upper_bound : expr) => (# [test] # [cfg (feature = "safe_api")] fn test_normal_debug () { let public = format ! ("{:?}" , [0u8 ; $ upper_bound] . as_ref ()) ; let test_debug_contents = format ! ("{:?}" , $ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap ()) ; assert_eq ! (test_debug_contents . contains (& public) , true) ; })) ;
    };
}

test_normal_debug!();