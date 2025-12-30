// Generated macro for can_access_arguments (function)
macro_rules! Depcrate_tests_subscriptionscan_access_arguments {
() => {
// Module: crate::tests::subscriptions
// Provides: {"can_access_arguments"}
// Dependencies: {}
# [test] fn can_access_arguments () { let query = r#"subscription {
            humanWithArgs(id: "123", name: "args name") {
                id
                name
              }
        }"# ; let (names , collected_values) = create_and_execute (query . into ()) . expect ("Got error from stream") ; let mut iterator_count = 0 ; let expected_values = vec ! [vec ! [Ok (Value :: Object (Object :: from_iter (iter :: from_fn (move || { iterator_count += 1 ; match iterator_count { 1 => Some (("id" , graphql :: value ! ("123"))) , 2 => Some (("name" , graphql :: value ! ("args name"))) , _ => None , } } ,))))]] ; assert_eq ! (names , vec ! ["humanWithArgs"]) ; assert_eq ! (collected_values , expected_values) ; }
};
}
