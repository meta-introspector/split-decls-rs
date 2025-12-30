// Generated macro for test_introspection_specific_interface_type_name_and_kind (function)
macro_rules! Depcrate_tests_introspection_teststest_introspection_specific_interface_type_name_and_kind {
() => {
// Module: crate::tests::introspection_tests
// Provides: {"test_introspection_specific_interface_type_name_and_kind"}
// Dependencies: {}
# [tokio :: test] async fn test_introspection_specific_interface_type_name_and_kind () { let doc = r#"
        query IntrospectionDroidKindQuery {
          __type(name: "Character") {
            name
            kind
          }
        }
        "# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql_value ! ({ "__type" : { "name" : "Character" , "kind" : "INTERFACE" , } }) , vec ! []))) ; }
};
}
