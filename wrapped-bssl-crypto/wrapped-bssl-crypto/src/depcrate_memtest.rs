// Generated macro for test (module)
macro_rules! Depcrate_memtest {
() => {
// Module: crate::mem
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_different_length () { assert ! (! constant_time_compare (& [0 , 1 , 2] , & [0])) } # [test] fn test_same_length_different_content () { assert ! (! constant_time_compare (& [0 , 1 , 2] , & [1 , 2 , 3])) } # [test] fn test_same_content () { assert ! (constant_time_compare (& [0 , 1 , 2] , & [0 , 1 , 2])) } # [test] fn test_empty_slices () { assert ! (constant_time_compare (& [] , & [])) } # [test] fn test_empty_slices_different () { assert ! (! constant_time_compare (& [] , & [0 , 1 , 2])) } }
};
}
