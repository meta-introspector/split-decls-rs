// Generated macro for test_join (function)
macro_rules! Depcrate_teststest_join {
() => {
// Module: crate::tests
// Provides: {"test_join"}
// Dependencies: {}
# [test] fn test_join () { assert_components (& ["foo" , "bar" , "baz"] , & rp ("foo/bar") . join ("baz///")) ; assert_components (& ["hello" , "world" , "foo" , "bar" , "baz"] , & rp ("hello/world") . join ("///foo/bar/baz") ,) ; assert_components (& ["foo" , "bar" , "baz"] , & rp ("") . join ("foo/bar/baz")) ; }
};
}
