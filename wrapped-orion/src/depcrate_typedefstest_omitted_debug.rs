// Generated macro for test_omitted_debug (macro)
macro_rules! Depcrate_typedefstest_omitted_debug {
() => {
// Module: crate::typedefs
// Provides: {"test_omitted_debug"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "safe_api")] macro_rules ! test_omitted_debug (($ name : ident , $ upper_bound : expr) => (# [test] # [cfg (feature = "safe_api")] fn test_omitted_debug () { let secret = format ! ("{:?}" , [0u8 ; $ upper_bound] . as_ref ()) ; let test_debug_contents = format ! ("{:?}" , $ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap ()) ; assert ! (! test_debug_contents . contains (& secret)) ; })) ;
};
}
