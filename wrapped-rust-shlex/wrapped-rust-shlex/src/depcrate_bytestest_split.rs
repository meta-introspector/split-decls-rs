// Generated macro for test_split (function)
macro_rules! Depcrate_bytestest_split {
() => {
// Module: crate::bytes
// Provides: {"test_split"}
// Dependencies: {}
# [test] fn test_split () { for & (input , output) in SPLIT_TEST_ITEMS { assert_eq ! (split (input) , output . map (| o | o . iter () . map (|& x | x . to_owned ()) . collect ())) ; } }
};
}
