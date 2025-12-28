macro_rules! test_omitted_debug {
    () => {
        # [cfg (test)] # [cfg (feature = "safe_api")] macro_rules ! test_omitted_debug (($ name : ident , $ upper_bound : expr) => (# [test] # [cfg (feature = "safe_api")] fn test_omitted_debug () { let secret = format ! ("{:?}" , [0u8 ; $ upper_bound] . as_ref ()) ; let test_debug_contents = format ! ("{:?}" , $ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap ()) ; assert ! (! test_debug_contents . contains (& secret)) ; })) ;
    };
}

test_omitted_debug!();