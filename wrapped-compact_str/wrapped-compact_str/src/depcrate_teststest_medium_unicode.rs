// Generated macro for test_medium_unicode (function)
macro_rules! Depcrate_teststest_medium_unicode {
() => {
// Module: crate::tests
// Provides: {"test_medium_unicode"}
// Dependencies: {}
# [test] fn test_medium_unicode () { let strs = vec ! [("☕️👀😁🎉" , false) , ("🦀😀😃😄😁🦀" , false) ,] ; # [allow (unused_variables)] for (s , is_heap) in strs { let compact = CompactString :: new (s) ; assert_eq ! (compact , s) ; assert_eq ! (s , compact) ; # [cfg (target_pointer_width = "32")] let is_heap = true ; assert_eq ! (compact . is_heap_allocated () , is_heap) ; } }
};
}
