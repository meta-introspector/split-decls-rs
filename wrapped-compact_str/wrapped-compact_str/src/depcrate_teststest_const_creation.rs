// Generated macro for test_const_creation (function)
macro_rules! Depcrate_teststest_const_creation {
() => {
// Module: crate::tests
// Provides: {"test_const_creation"}
// Dependencies: {}
# [test] fn test_const_creation () { const EMPTY : CompactString = CompactString :: const_new ("") ; const SHORT : CompactString = CompactString :: const_new ("rust") ; const EMPTY_STATIC_STR : CompactString = CompactString :: const_new ("") ; const SHORT_STATIC_STR : CompactString = CompactString :: const_new ("rust") ; # [cfg (target_pointer_width = "64")] const PACKED : CompactString = CompactString :: const_new ("i am 24 characters long!") ; # [cfg (target_pointer_width = "32")] const PACKED : CompactString = CompactString :: const_new ("i am 12 char") ; const PACKED_STATIC_STR0 : CompactString = CompactString :: const_new ("i am 24 characters long!") ; const PACKED_STATIC_STR1 : CompactString = CompactString :: const_new ("i am 12 char") ; assert_eq ! (EMPTY , CompactString :: new ("")) ; assert_eq ! (SHORT , CompactString :: new ("rust")) ; assert_eq ! (EMPTY_STATIC_STR , CompactString :: new ("")) ; assert_eq ! (SHORT_STATIC_STR , CompactString :: new ("rust")) ; # [cfg (target_pointer_width = "64")] assert_eq ! (PACKED , CompactString :: new ("i am 24 characters long!")) ; # [cfg (target_pointer_width = "32")] assert_eq ! (PACKED , CompactString :: new ("i am 12 char")) ; assert_eq ! (PACKED_STATIC_STR0 , CompactString :: new ("i am 24 characters long!")) ; assert_eq ! (PACKED_STATIC_STR1 , CompactString :: new ("i am 12 char")) ; }
};
}
