// Generated macro for enum_introspection (function)
macro_rules! Depcrate_executor_tests_introspectionenum_introspection {
() => {
// Module: crate::executor_tests::introspection
// Provides: {"enum_introspection"}
// Dependencies: {}
# [tokio :: test] async fn enum_introspection () { let doc = r#"
    {
        __type(name: "SampleEnum") {
            name
            kind
            description
            enumValues {
                name
                description
                isDeprecated
                deprecationReason
            }
            interfaces { name }
            possibleTypes { name }
            inputFields { name }
            ofType { name }
        }
    }
    "# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let (result , errs) = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; let type_info = result . as_object_value () . expect ("Result is not an object") . get_field_value ("__type") . expect ("__type field missing") . as_object_value () . expect ("__type field not an object value") ; assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("SampleEnum")) ,) ; assert_eq ! (type_info . get_field_value ("kind") , Some (& graphql :: value ! ("ENUM")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (type_info . get_field_value ("interfaces") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (type_info . get_field_value ("possibleTypes") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (type_info . get_field_value ("inputFields") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (type_info . get_field_value ("ofType") , Some (& graphql :: value ! (null))) ; let values = type_info . get_field_value ("enumValues") . expect ("enumValues field missing") . as_list_value () . expect ("enumValues not a list") ; assert_eq ! (values . len () , 2) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "ONE" , "description" : null , "isDeprecated" : false , "deprecationReason" : null , }))) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "TWO" , "description" : null , "isDeprecated" : false , "deprecationReason" : null , }))) ; }
};
}
