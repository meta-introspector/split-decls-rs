// Generated macro for test_remove_empty_string_static (function)
macro_rules! Depcrate_teststest_remove_empty_string_static {
() => {
// Module: crate::tests
// Provides: {"test_remove_empty_string_static"}
// Dependencies: {}
# [test] # [should_panic (expected = "cannot remove a char from the end of a string")] fn test_remove_empty_string_static () { let mut compact = CompactString :: const_new ("") ; compact . remove (0) ; }
};
}
