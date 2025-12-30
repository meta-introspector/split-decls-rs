// Generated macro for returns_error (function)
macro_rules! Depcrate_tests_subscriptionsreturns_error {
() => {
// Module: crate::tests::subscriptions
// Provides: {"returns_error"}
// Dependencies: {}
# [test] fn returns_error () { let query = r#"subscription {
        errorHuman(id: "1") {
            id
            name
        }
    }"# ; let response = create_and_execute (query . into ()) ; assert ! (response . is_err ()) ; let returned_errors = response . err () . unwrap () ; let expected_error = ExecutionError :: new (crate :: parser :: SourcePosition :: new (23 , 1 , 8) , & ["errorHuman"] , FieldError :: new ("handler error" , graphql :: value ! ("more details")) ,) ; assert_eq ! (returned_errors , vec ! [expected_error]) ; }
};
}
