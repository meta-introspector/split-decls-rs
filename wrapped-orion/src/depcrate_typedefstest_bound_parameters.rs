// Generated macro for test_bound_parameters (macro)
macro_rules! Depcrate_typedefstest_bound_parameters {
() => {
// Module: crate::typedefs
// Provides: {"test_bound_parameters"}
// Dependencies: {}
# [cfg (test)] macro_rules ! test_bound_parameters (($ name : ident , $ lower_bound : expr , $ upper_bound : expr , $ gen_length : expr) => (# [test] fn test_bound_params () { assert ! ($ lower_bound <= $ upper_bound) ; assert ! ($ gen_length <= $ upper_bound) ; assert ! ($ gen_length >= $ lower_bound) ; })) ;
};
}
