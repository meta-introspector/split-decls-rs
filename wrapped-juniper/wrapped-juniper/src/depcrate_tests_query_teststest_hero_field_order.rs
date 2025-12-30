// Generated macro for test_hero_field_order (function)
macro_rules! Depcrate_tests_query_teststest_hero_field_order {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_hero_field_order"}
// Dependencies: {}
# [tokio :: test] async fn test_hero_field_order () { let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; let doc = r#"{
        hero {
            id
            name
        }
    }"# ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "hero" : { "id" : "2001" , "name" : "R2-D2" } }) , vec ! [] ,)) ,) ; let doc_reversed = r#"{
        hero {
            name
            id
        }
    }"# ; assert_eq ! (crate :: execute (doc_reversed , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "hero" : { "name" : "R2-D2" , "id" : "2001" } }) , vec ! [] ,)) ,) ; }
};
}
