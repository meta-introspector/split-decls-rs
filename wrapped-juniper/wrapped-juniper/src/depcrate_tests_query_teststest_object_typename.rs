// Generated macro for test_object_typename (function)
macro_rules! Depcrate_tests_query_teststest_object_typename {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_object_typename"}
// Dependencies: {}
# [tokio :: test] async fn test_object_typename () { let doc = r#"{
        human(id: "1000") {
            __typename
        }
    }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "human" : { "__typename" : "Human" } }) , vec ! [])) ,) ; }
};
}
