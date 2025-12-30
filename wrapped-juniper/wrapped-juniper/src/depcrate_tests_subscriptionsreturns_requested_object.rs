// Generated macro for returns_requested_object (function)
macro_rules! Depcrate_tests_subscriptionsreturns_requested_object {
() => {
// Module: crate::tests::subscriptions
// Provides: {"returns_requested_object"}
// Dependencies: {}
# [test] fn returns_requested_object () { let query = r#"subscription {
        asyncHuman(id: "1") {
            id
            name
        }
    }"# ; let (names , collected_values) = create_and_execute (query . into ()) . expect ("Got error from stream") ; let mut iterator_count = 0 ; let expected_values = vec ! [vec ! [Ok (Value :: Object (Object :: from_iter (std :: iter :: from_fn (move || { iterator_count += 1 ; match iterator_count { 1 => Some (("id" , graphql :: value ! ("stream id"))) , 2 => Some (("name" , graphql :: value ! ("stream name"))) , _ => None , } }) ,)))]] ; assert_eq ! (names , vec ! ["asyncHuman"]) ; assert_eq ! (collected_values , expected_values) ; }
};
}
