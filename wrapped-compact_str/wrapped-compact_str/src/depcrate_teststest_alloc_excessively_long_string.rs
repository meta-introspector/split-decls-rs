// Generated macro for test_alloc_excessively_long_string (function)
macro_rules! Depcrate_teststest_alloc_excessively_long_string {
() => {
// Module: crate::tests
// Provides: {"test_alloc_excessively_long_string"}
// Dependencies: {}
# [cfg (not (debug_assertions))] # [cfg (target_pointer_width = "64")] # [test] # [should_panic = "Cannot allocate memory to hold CompactString"] fn test_alloc_excessively_long_string () { std :: hint :: black_box (CompactString :: with_capacity ((1 << 56) - 2)) ; }
};
}
