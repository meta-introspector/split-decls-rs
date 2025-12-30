// Generated macro for can_access_context (function)
macro_rules! Depcrate_tests_subscriptionscan_access_context {
() => {
// Module: crate::tests::subscriptions
// Provides: {"can_access_context"}
// Dependencies: {}
# [test] fn can_access_context () { let query = r#"subscription {
            humanWithContext {
                id
              }
        }"# ; let (names , collected_values) = create_and_execute (query . into ()) . expect ("Got error from stream") ; let mut iterator_count = 0 ; let expected_values = vec ! [vec ! [Ok (Value :: Object (Object :: from_iter (iter :: from_fn (move || { iterator_count += 1 ; match iterator_count { 1 => Some (("id" , graphql :: value ! ("2"))) , _ => None , } } ,))))]] ; assert_eq ! (names , vec ! ["humanWithContext"]) ; assert_eq ! (collected_values , expected_values) ; }
};
}
