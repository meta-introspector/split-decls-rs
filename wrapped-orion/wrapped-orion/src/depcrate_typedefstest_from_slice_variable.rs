// Generated macro for test_from_slice_variable (macro)
macro_rules! Depcrate_typedefstest_from_slice_variable {
() => {
// Module: crate::typedefs
// Provides: {"test_from_slice_variable"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "safe_api")] macro_rules ! test_from_slice_variable (($ name : ident) => (# [test] # [cfg (feature = "safe_api")] fn test_from_slice_variable () { assert ! ($ name :: from_slice (& [0u8 ; 512]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; 256]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; 1]) . is_ok ()) ; assert ! ($ name :: from_slice (& [0u8 ; 0]) . is_err ()) ; })) ;
};
}
