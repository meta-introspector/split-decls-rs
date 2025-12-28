macro_rules! test_bound_parameters {
    () => {
        # [cfg (test)] macro_rules ! test_bound_parameters (($ name : ident , $ lower_bound : expr , $ upper_bound : expr , $ gen_length : expr) => (# [test] fn test_bound_params () { assert ! ($ lower_bound <= $ upper_bound) ; assert ! ($ gen_length <= $ upper_bound) ; assert ! ($ gen_length >= $ lower_bound) ; })) ;
    };
}

test_bound_parameters!();