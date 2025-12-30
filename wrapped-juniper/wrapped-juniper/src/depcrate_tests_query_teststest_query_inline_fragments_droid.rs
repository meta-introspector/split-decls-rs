// Generated macro for test_query_inline_fragments_droid (function)
macro_rules! Depcrate_tests_query_teststest_query_inline_fragments_droid {
() => {
// Module: crate::tests::query_tests
// Provides: {"test_query_inline_fragments_droid"}
// Dependencies: {}
# [tokio :: test] async fn test_query_inline_fragments_droid () { let doc = r#"query InlineFragments {
        hero {
            name
            __typename

            ...on Droid {
                primaryFunction
            }
        }
    }"# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; assert_eq ! (crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await , Ok ((graphql :: value ! ({ "hero" : { "__typename" : "Droid" , "name" : "R2-D2" , "primaryFunction" : "Astromech" , } }) , vec ! [] ,)) ,) ; }
};
}
