// Generated macro for test_query_name_variable (function)
macro_rules! Depcrate_tests_query_teststest_query_name_variable {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_query_name_variable"}
// Dependencies: {}
# [tokio :: test] async fn test_query_name_variable () { let doc = r#"query FetchSomeIDQuery($someId: String!) { human(id: $someId) { name } }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; let vars = graphql :: vars ! { "someId" : "1000" } ; assert_eq ! (crate :: execute (doc , None , & schema , & vars , & database) . await , Ok ((graphql :: value ! ({ "human" : { "name" : "Luke Skywalker" } }) , vec ! [] ,)) ,) ; }
};
}
