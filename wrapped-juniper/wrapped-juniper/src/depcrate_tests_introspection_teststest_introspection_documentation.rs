// Generated macro for test_introspection_documentation (function)
macro_rules! Depcrate_tests_introspection_teststest_introspection_documentation {
() => {
// Module: crate::tests::introspection_tests
// Provides: {"test_introspection_documentation"}
// Dependencies: {}
# [tokio :: test] async fn test_introspection_documentation () { let doc = r#"
        query IntrospectionDroidDescriptionQuery {
          __type(name: "Droid") {
            name
            description
          }
        }
        "# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql_value ! ({ "__type" : { "name" : "Droid" , "description" : "A mechanical creature in the Star Wars universe." , } , }) , vec ! []))) ; }
};
}
