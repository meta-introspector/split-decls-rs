// Generated macro for test_split (function)
macro_rules! Depcratetest_split {
() => {
// Module: crate
// Provides: {"test_split"}
// Dependencies: {}
# [test] fn test_split () { for & (input , output) in SPLIT_TEST_ITEMS { assert_eq ! (split (input) , output . map (| o | o . iter () . map (|& x | x . to_owned ()) . collect ())) ; } }
};
}
