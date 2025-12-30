// Generated macro for test_remove_empty_string (function)
macro_rules! Depcrate_teststest_remove_empty_string {
() => {
// Module: crate::tests
// Provides: {"test_remove_empty_string"}
// Dependencies: {}
# [test] # [should_panic (expected = "cannot remove a char from the end of a string")] fn test_remove_empty_string () { let mut compact = CompactString :: new ("") ; compact . remove (0) ; }
};
}
