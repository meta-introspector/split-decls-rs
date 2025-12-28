macro_rules! test_from_slice {
    () => {
        # [cfg (test)] macro_rules ! test_from_slice (($ name : ident , $ lower_bound : expr , $ upper_bound : expr) => (# [test] fn test_from_slice () { assert ! ($ name :: from_slice (& [0u8 ; $ upper_bound]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; $ lower_bound]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; $ upper_bound + 1]) . is_err ()) ; assert ! ($ name :: from_slice (& [0u8 ; $ lower_bound - 1]) . is_err ()) ; assert ! ($ name :: from_slice (& [0u8 ; 0]) . is_err ()) ; if $ upper_bound != $ lower_bound { assert ! ($ name :: from_slice (& [0u8 ; $ upper_bound - 1]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; $ lower_bound + 1]) . is_ok ()) ; } })) ;
    };
}

test_from_slice!();