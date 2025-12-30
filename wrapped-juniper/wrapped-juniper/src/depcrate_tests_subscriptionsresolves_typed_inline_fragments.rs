// Generated macro for resolves_typed_inline_fragments (function)
macro_rules! Depcrate_tests_subscriptionsresolves_typed_inline_fragments {
() => {
// Module: crate::tests::subscriptions
// Provides: {"resolves_typed_inline_fragments"}
// Dependencies: {}
# [test] fn resolves_typed_inline_fragments () { let query = r#"subscription {
             ... on MySubscription {
                asyncHuman(id: "32") {
                  id
                }
             }
           }"# ; let (names , collected_values) = create_and_execute (query . into ()) . expect ("Got error from stream") ; let mut iterator_count = 0 ; let expected_values = vec ! [vec ! [Ok (Value :: Object (Object :: from_iter (iter :: from_fn (move || { iterator_count += 1 ; match iterator_count { 1 => Some (("id" , graphql :: value ! ("stream id"))) , _ => None , } } ,))))]] ; assert_eq ! (names , vec ! ["asyncHuman"]) ; assert_eq ! (collected_values , expected_values) ; }
};
}
