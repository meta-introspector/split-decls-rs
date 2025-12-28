macro_rules! test_partial_eq {
    () => {
        # [cfg (test)] macro_rules ! test_partial_eq (($ name : ident , $ upper_bound : expr) => (# [test] fn test_partial_eq () { assert_eq ! ($ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap () , $ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap ()) ; assert_ne ! ($ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap () , $ name :: from_slice (& [1u8 ; $ upper_bound]) . unwrap ()) ; assert_eq ! ($ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap () , [0u8 ; $ upper_bound] . as_ref ()) ; assert_ne ! ($ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap () , [1u8 ; $ upper_bound] . as_ref ()) ; })) ;
    };
}

test_partial_eq!()