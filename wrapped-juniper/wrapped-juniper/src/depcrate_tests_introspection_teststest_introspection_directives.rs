// Generated macro for test_introspection_directives (function)
macro_rules! Depcrate_tests_introspection_teststest_introspection_directives {
() => {
// Module: crate::tests::introspection_tests
// Provides: {"test_introspection_directives"}
// Dependencies: {}
# [tokio :: test] async fn test_introspection_directives () { let q = r#"
        query IntrospectionQuery {
          __schema {
            directives {
              name
              locations
            }
          }
        }
    "# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; let result = crate :: execute (q , None , & schema , & graphql :: vars ! { } , & database) . await . unwrap () ; let expected : Value = graphql_value ! ({ "__schema" : { "directives" : [{ "name" : "deprecated" , "locations" : ["FIELD_DEFINITION" , "ARGUMENT_DEFINITION" , "INPUT_FIELD_DEFINITION" , "ENUM_VALUE" ,] , } , { "name" : "include" , "locations" : ["FIELD" , "FRAGMENT_SPREAD" , "INLINE_FRAGMENT" ,] , } , { "name" : "oneOf" , "locations" : ["INPUT_OBJECT" ,] , } , { "name" : "skip" , "locations" : ["FIELD" , "FRAGMENT_SPREAD" , "INLINE_FRAGMENT" ,] , } , { "name" : "specifiedBy" , "locations" : ["SCALAR" ,] , } ,] , } , }) ; assert_eq ! (serde_json :: to_string_pretty (& result . 0) . unwrap () , serde_json :: to_string_pretty (& expected) . unwrap () ,) ; }
};
}
