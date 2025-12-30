// Generated macro for test_query_alias_multiple_with_fragment (function)
macro_rules! Depcrate_tests_query_teststest_query_alias_multiple_with_fragment {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_query_alias_multiple_with_fragment"}
// Dependencies: {}
# [tokio :: test] async fn test_query_alias_multiple_with_fragment () { let doc = r#"
        query UseFragment {
            luke: human(id: "1000") { ...HumanFragment }
            leia: human(id: "1003") { ...HumanFragment }
        }

        fragment HumanFragment on Human {
            name
            homePlanet
        }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "luke" : { "name" : "Luke Skywalker" , "homePlanet" : "Tatooine" } , "leia" : { "name" : "Leia Organa" , "homePlanet" : "Alderaan" } , }) , vec ! [] ,)) ,) ; }
};
}
