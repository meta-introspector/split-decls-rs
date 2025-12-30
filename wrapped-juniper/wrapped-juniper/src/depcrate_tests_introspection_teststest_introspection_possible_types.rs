// Generated macro for test_introspection_possible_types (function)
macro_rules! Depcrate_tests_introspection_teststest_introspection_possible_types {
() => {
// Module: crate::tests::introspection_tests
// Provides: {"test_introspection_possible_types"}
// Dependencies: {}
# [tokio :: test] async fn test_introspection_possible_types () { let doc = r#"
        query IntrospectionDroidDescriptionQuery {
          __type(name: "Character") {
            possibleTypes {
              name
            }
          }
        }
        "# ; let database = Database :: new () ; let schema = RootNode :: new (Query , EmptyMutation :: < Database > :: new () , EmptySubscription :: < Database > :: new () ,) ; let result = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & database) . await ; let (result , errors) = result . ok () . expect ("Query returned error") ; assert_eq ! (errors , vec ! []) ; let possible_types = result . as_object_value () . expect ("execution result not an object") . get_field_value ("__type") . expect ("'__type' not present in result") . as_object_value () . expect ("'__type' not an object") . get_field_value ("possibleTypes") . expect ("'possibleTypes' not present in '__type'") . as_list_value () . expect ("'possibleTypes' not a list") . iter () . map (| t | { t . as_object_value () . expect ("possible type not an object") . get_field_value ("name") . expect ("'name' not present in type") . as_scalar () . and_then (| s | s . try_as_str ()) . expect ("'name' not a string") }) . collect :: < HashSet < _ > > () ; assert_eq ! (possible_types , vec ! ["Human" , "Droid"] . into_iter () . collect ()) ; }
};
}
