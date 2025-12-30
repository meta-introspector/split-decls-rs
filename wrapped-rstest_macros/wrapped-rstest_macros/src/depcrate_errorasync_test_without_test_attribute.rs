// Generated macro for async_test_without_test_attribute (function)
macro_rules! Depcrate_errorasync_test_without_test_attribute {
() => {
// Module: crate::error
// Provides: {"async_test_without_test_attribute"}
// Dependencies: {}
fn async_test_without_test_attribute < 'a > (test : & 'a ItemFn , arguments : & 'a ArgumentsInfo) -> Errors < 'a > { if test . sig . asyncness . is_some () && arguments . test_attr () . is_none () { let span = test . sig . ident . span () ; Box :: new (std :: iter :: once (syn :: Error :: new (span , "async test requires either explicit `test_attr` or implicit (attribute path ends with `test`)"))) } else { Box :: new (std :: iter :: empty ()) } }
};
}
