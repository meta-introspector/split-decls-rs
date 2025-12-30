// Generated macro for test_attributes (function)
macro_rules! Depcrate_errortest_attributes {
() => {
// Module: crate::error
// Provides: {"test_attributes"}
// Dependencies: {}
fn test_attributes < 'a > (test : & 'a ItemFn , arguments : & 'a ArgumentsInfo) -> Errors < 'a > { Box :: new (async_test_without_test_attribute (test , arguments) . chain (malformed_explicit_test_attr (test))) }
};
}
