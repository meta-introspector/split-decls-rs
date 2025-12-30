// Generated macro for test_query_friends_names (function)
macro_rules! Depcrate_tests_query_teststest_query_friends_names {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_query_friends_names"}
// Dependencies: {}
# [tokio :: test] async fn test_query_friends_names () { let doc = r#"{ human(id: "1000") { friends { name } } }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "human" : { "friends" : [{ "name" : "Han Solo" } , { "name" : "Leia Organa" } , { "name" : "C-3PO" } , { "name" : "R2-D2" } ,] , } }) , vec ! [] ,)) ,) ; }
};
}
