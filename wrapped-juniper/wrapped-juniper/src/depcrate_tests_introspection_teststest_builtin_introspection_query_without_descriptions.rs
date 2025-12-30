// Generated macro for test_builtin_introspection_query_without_descriptions (function)
macro_rules! Depcrate_tests_introspection_teststest_builtin_introspection_query_without_descriptions {
() => {
// Module: crate::tests::introspection_tests
// Provides: {"test_builtin_introspection_query_without_descriptions"}
// Dependencies: {}
# [tokio :: test] async fn test_builtin_introspection_query_without_descriptions () { let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; let result = crate :: introspect (& schema , & database , IntrospectionFormat :: WithoutDescriptions) . unwrap () ; let expected = schema_introspection_result_without_descriptions () ; assert_eq ! (serde_json :: to_string_pretty (& result . 0) . unwrap () , serde_json :: to_string_pretty (& expected) . unwrap () ,) ; }
};
}
