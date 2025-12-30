// Generated macro for interface_inline_fragment_friends (function)
macro_rules! Depcrate_tests_query_testsinterface_inline_fragment_friends {
() => {
// Module: crate::tests::query_tests
// Provides: {"interface_inline_fragment_friends"}
// Dependencies: {}
# [tokio :: test] async fn interface_inline_fragment_friends () { let doc = r#"{
        human(id: "1002") {
            friends {
                name
                ... on Human { homePlanet }
                ... on Droid { primaryFunction }
            }
        }
    }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "human" : { "friends" : [{ "name" : "Luke Skywalker" , "homePlanet" : "Tatooine" } , { "name" : "Leia Organa" , "homePlanet" : "Alderaan" } , { "name" : "R2-D2" , "primaryFunction" : "Astromech" } ,] , } }) , vec ! [] ,))) ; }
};
}
