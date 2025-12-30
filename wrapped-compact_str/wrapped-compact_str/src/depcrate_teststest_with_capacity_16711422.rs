// Generated macro for test_with_capacity_16711422 (function)
macro_rules! Depcrate_teststest_with_capacity_16711422 {
() => {
// Module: crate::tests
// Provides: {"test_with_capacity_16711422"}
// Dependencies: {}
# [test] fn test_with_capacity_16711422 () { assert_eq ! (16711422_u32 . to_le_bytes () , [254 , 254 , 254 , 0]) ; let compact = CompactString :: with_capacity (16711422) ; let std_str = String :: with_capacity (16711422) ; assert ! (compact . is_heap_allocated ()) ; assert_eq ! (compact . capacity () , std_str . capacity ()) ; assert_eq ! (compact , "") ; assert_eq ! (compact , std_str) ; }
};
}
