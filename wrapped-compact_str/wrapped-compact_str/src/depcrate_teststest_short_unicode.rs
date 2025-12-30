// Generated macro for test_short_unicode (function)
macro_rules! Depcrate_teststest_short_unicode {
() => {
// Module: crate::tests
// Provides: {"test_short_unicode"}
// Dependencies: {}
# [test] fn test_short_unicode () { let strs = vec ! [("🦀" , false) , ("🌧☀️" , false) , ("咬𓅈ꁈ:_" , false) ,] ; for (s , is_heap) in strs { let compact = CompactString :: new (s) ; assert_eq ! (compact , s) ; assert_eq ! (s , compact) ; assert_eq ! (compact . is_heap_allocated () , is_heap) ; } }
};
}
