// Generated macro for test_query_alias_multiple (function)
macro_rules! Depcrate_tests_query_teststest_query_alias_multiple {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_query_alias_multiple"}
// Dependencies: {}
# [tokio :: test] async fn test_query_alias_multiple () { let doc = r#"{
        luke: human(id: "1000") { name }
        leia: human(id: "1003") { name }
    }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "luke" : { "name" : "Luke Skywalker" } , "leia" : { "name" : "Leia Organa" } , }) , vec ! [] ,)) ,) ; }
};
}
