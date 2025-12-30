// Generated macro for test_introspection_type_name (function)
macro_rules! Depcrate_tests_introspection_teststest_introspection_type_name {
() => {
// Module: crate::tests::introspection_tests
// Provides: {"test_introspection_type_name"}
// Dependencies: {}
# [tokio :: test] async fn test_introspection_type_name () { let doc = r#"
        query IntrospectionQueryTypeQuery {
          __type(name: "Droid") {
            name
          }
        }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql_value ! ({ "__type" : { "name" : "Droid" , } , }) , vec ! []))) ; }
};
}
