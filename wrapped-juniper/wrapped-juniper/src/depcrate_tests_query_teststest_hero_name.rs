// Generated macro for test_hero_name (function)
macro_rules! Depcrate_tests_query_teststest_hero_name {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_hero_name"}
// Dependencies: {}
# [tokio :: test] async fn test_hero_name () { let doc = r#"{
        hero {
            name
        }
    }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "hero" : { "name" : "R2-D2" } }) , vec ! [])) ,) ; }
};
}
