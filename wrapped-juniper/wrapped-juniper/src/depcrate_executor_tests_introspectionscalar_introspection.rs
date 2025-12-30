// Generated macro for scalar_introspection (function)
macro_rules! Depcrate_executor_tests_introspectionscalar_introspection {
() => {
// Module: crate::executor_tests::introspection
// Provides: {"scalar_introspection"}
// Dependencies: {}
# [tokio :: test] async fn scalar_introspection () { let doc = r#"
    {
        __type(name: "SampleScalar") {
            name
            kind
            description
            specifiedByURL
            fields { name }
            interfaces { name }
            possibleTypes { name }
            enumValues { name }
            inputFields { name }
            ofType { name }
        }
    }
    "# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let (result , errs) = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; let type_info = result . as_object_value () . expect ("Result is not an object") . get_field_value ("__type") . expect ("__type field missing") ; assert_eq ! (type_info , & graphql :: value ! ({ "name" : "SampleScalar" , "kind" : "SCALAR" , "description" : null , "specifiedByURL" : null , "fields" : null , "interfaces" : null , "possibleTypes" : null , "enumValues" : null , "inputFields" : null , "ofType" : null , }) ,) ; }
};
}
