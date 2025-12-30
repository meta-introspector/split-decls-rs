// Generated macro for test_collect (function)
macro_rules! Depcrate_tests_datatest_collect {
() => {
// Module: crate::tests::data
// Provides: {"test_collect"}
// Dependencies: {}
# [cfg (feature = "block2")] # [test] fn test_collect () { let bytes = [3 , 7 , 16 , 52 , 112 , 19] ; let data : objc2 :: rc :: Retained < NSData > = bytes . into_iter () . collect () ; assert_eq ! (format ! ("{data:?}") , "[3, 7, 16, 52, 112, 19]") ; }
};
}
