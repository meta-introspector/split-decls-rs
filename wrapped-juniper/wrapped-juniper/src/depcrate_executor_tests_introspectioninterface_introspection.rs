// Generated macro for interface_introspection (function)
macro_rules! Depcrate_executor_tests_introspectioninterface_introspection {
() => {
// Module: crate::executor_tests::introspection
// Provides: {"interface_introspection"}
// Dependencies: {}
# [tokio :: test] async fn interface_introspection () { let doc = r#"
    {
        __type(name: "SampleInterface") {
            name
            kind
            description
            possibleTypes {
                name
            }
            fields {
                name
                description
                args {
                    name
                }
                type {
                    name
                    kind
                    ofType {
                        name
                        kind
                    }
                }
                isDeprecated
                deprecationReason
            }
            interfaces { name }
            enumValues { name }
            inputFields { name }
            ofType { name }
        }
    }
    "# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let (result , errs) = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; let type_info = result . as_object_value () . expect ("Result is not an object") . get_field_value ("__type") . expect ("__type field missing") . as_object_value () . expect ("__type field not an object value") ; assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("SampleInterface")) ,) ; assert_eq ! (type_info . get_field_value ("kind") , Some (& graphql :: value ! ("INTERFACE")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! ("A sample interface")) ,) ; assert_eq ! (type_info . get_field_value ("interfaces") , Some (& graphql :: value ! ([])) ,) ; assert_eq ! (type_info . get_field_value ("enumValues") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (type_info . get_field_value ("inputFields") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (type_info . get_field_value ("ofType") , Some (& graphql :: value ! (null))) ; let possible_types = type_info . get_field_value ("possibleTypes") . expect ("possibleTypes field missing") . as_list_value () . expect ("possibleTypes not a list") ; assert_eq ! (possible_types . len () , 1) ; assert ! (possible_types . contains (& graphql :: value ! ({ "name" : "Root" }))) ; let fields = type_info . get_field_value ("fields") . expect ("fields field missing") . as_list_value () . expect ("fields field not an object value") ; assert_eq ! (fields . len () , 1) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "sampleEnum" , "description" : "A sample field in the interface" , "args" : [] , "type" : { "name" : null , "kind" : "NON_NULL" , "ofType" : { "name" : "SampleEnum" , "kind" : "ENUM" , } , } , "isDeprecated" : false , "deprecationReason" : null , }))) ; }
};
}
