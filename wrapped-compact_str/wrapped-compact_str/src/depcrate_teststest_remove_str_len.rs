// Generated macro for test_remove_str_len (function)
macro_rules! Depcrate_teststest_remove_str_len {
() => {
// Module: crate::tests
// Provides: {"test_remove_str_len"}
// Dependencies: {}
# [test] # [should_panic (expected = "cannot remove a char from the end of a string")] fn test_remove_str_len () { let mut compact = CompactString :: new ("hello world") ; compact . remove (compact . len ()) ; }
};
}
