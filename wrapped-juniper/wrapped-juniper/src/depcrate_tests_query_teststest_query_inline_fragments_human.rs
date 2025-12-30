// Generated macro for test_query_inline_fragments_human (function)
macro_rules! Depcrate_tests_query_teststest_query_inline_fragments_human {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_query_inline_fragments_human"}
// Dependencies: {}
# [tokio :: test] async fn test_query_inline_fragments_human () { let doc = r#"query InlineFragments {
        hero(episode: EMPIRE) {
            __typename
            name
        }
    }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "hero" : { "__typename" : "Human" , "name" : "Luke Skywalker" , } }) , vec ! [] ,)) ,) ; }
};
}
