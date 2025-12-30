// Generated macro for test_name_is_valid (function)
macro_rules! Depcrate_types_nametest_name_is_valid {
() => {
// Module: crate::types::name
// Provides: {"test_name_is_valid"}
// Dependencies: {}
# [test] fn test_name_is_valid () { assert ! (Name :: is_valid ("Foo")) ; assert ! (Name :: is_valid ("foo42")) ; assert ! (Name :: is_valid ("_Foo")) ; assert ! (Name :: is_valid ("_Foo42")) ; assert ! (Name :: is_valid ("_foo42")) ; assert ! (Name :: is_valid ("_42Foo")) ; assert ! (! Name :: is_valid ("42_Foo")) ; assert ! (! Name :: is_valid ("Foo-42")) ; assert ! (! Name :: is_valid ("Foo???")) ; }
};
}
