// Generated macro for custom_domain (function)
macro_rules! Depcrate_tests_errorcustom_domain {
() => {
// Module: crate::tests::error
// Provides: {"custom_domain"}
// Dependencies: {}
# [test] fn custom_domain () { let error = NSError :: new (42 , ns_string ! ("MyDomain")) ; assert_eq ! (error . code () , 42) ; assert_eq ! (&* error . domain () , ns_string ! ("MyDomain")) ; let expected = if cfg ! (target_vendor = "apple") { "The operation couldn’t be completed. (MyDomain error 42.)" } else { "MyDomain 42" } ; assert_eq ! (format ! ("{error}") , expected) ; }
};
}
