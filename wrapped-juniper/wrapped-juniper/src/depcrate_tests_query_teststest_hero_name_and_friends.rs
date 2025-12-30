// Generated macro for test_hero_name_and_friends (function)
macro_rules! Depcrate_tests_query_teststest_hero_name_and_friends {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_hero_name_and_friends"}
// Dependencies: {}
# [tokio :: test] async fn test_hero_name_and_friends () { let doc = r#"{
        hero {
            id
            name
            friends {
                name
            }
        }
    }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "hero" : { "id" : "2001" , "name" : "R2-D2" , "friends" : [{ "name" : "Luke Skywalker" } , { "name" : "Han Solo" } , { "name" : "Leia Organa" } ,] , } }) , vec ! [] ,)) ,) ; }
};
}
