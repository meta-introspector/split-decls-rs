// Generated macro for type_alias (function)
macro_rules! Depcrate_tests_subscriptionstype_alias {
() => {
// Module: crate::tests::subscriptions
// Provides: {"type_alias"}
// Dependencies: {}
# [test] fn type_alias () { let query = r#"subscription {
        aliasedHuman: asyncHuman(id: "1") {
            id
            name
        }
    }"# ; let (names , collected_values) = create_and_execute (query . into ()) . expect ("Got error from stream") ; let mut iterator_count = 0 ; let expected_values = vec ! [vec ! [Ok (Value :: Object (Object :: from_iter (iter :: from_fn (move || { iterator_count += 1 ; match iterator_count { 1 => Some (("id" , graphql :: value ! ("stream id"))) , 2 => Some (("name" , graphql :: value ! ("stream name"))) , _ => None , } } ,))))]] ; assert_eq ! (names , vec ! ["aliasedHuman"]) ; assert_eq ! (collected_values , expected_values) ; }
};
}
