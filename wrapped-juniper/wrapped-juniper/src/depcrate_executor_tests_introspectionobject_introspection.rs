// Generated macro for object_introspection (function)
macro_rules! Depcrate_executor_tests_introspectionobject_introspection {
() => {
// Module: crate::executor_tests::introspection
// Provides: {"object_introspection"}
// Dependencies: {}
# [tokio :: test] async fn object_introspection () { let doc = r#"
    {
        __type(name: "Root") {
            name
            kind
            description
            fields {
                name
                description
                args {
                    name
                    description
                    type {
                        name
                        kind
                        ofType {
                            name
                            kind
                            ofType {
                                name
                            }
                        }
                    }
                    defaultValue
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
            possibleTypes { name }
            interfaces { name }
            enumValues { name }
            inputFields { name }
            ofType { name }
        }
    }
    "# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let (result , errs) = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; let type_info = result . as_object_value () . expect ("Result is not an object") . get_field_value ("__type") . expect ("__type field missing") . as_object_value () . expect ("__type field not an object value") ; assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("Root")) ,) ; assert_eq ! (type_info . get_field_value ("kind") , Some (& graphql :: value ! ("OBJECT")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! ("The root query object in the schema")) ,) ; assert_eq ! (type_info . get_field_value ("interfaces") , Some (& graphql :: value ! ([{ "name" : "SampleInterface" }])) ,) ; assert_eq ! (type_info . get_field_value ("enumValues") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (type_info . get_field_value ("inputFields") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (type_info . get_field_value ("ofType") , Some (& graphql :: value ! (null))) ; assert_eq ! (type_info . get_field_value ("possibleTypes") , Some (& graphql :: value ! (null)) ,) ; let fields = type_info . get_field_value ("fields") . expect ("fields field missing") . as_list_value () . expect ("fields field not an object value") ; assert_eq ! (fields . len () , 2) ; println ! ("Fields: {fields:#?}") ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "sampleEnum" , "description" : null , "args" : [] , "type" : { "name" : null , "kind" : "NON_NULL" , "ofType" : { "name" : "SampleEnum" , "kind" : "ENUM" , } , } , "isDeprecated" : false , "deprecationReason" : null , }))) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "sampleScalar" , "description" : "A sample scalar field on the object" , "args" : [{ "name" : "first" , "description" : "The first number" , "type" : { "name" : null , "kind" : "NON_NULL" , "ofType" : { "name" : "Int" , "kind" : "SCALAR" , "ofType" : null , } , } , "defaultValue" : null , } , { "name" : "second" , "description" : "The second number" , "type" : { "name" : null , "kind" : "NON_NULL" , "ofType" : { "name" : "Int" , "kind" : "SCALAR" , "ofType" : null , } , } , "defaultValue" : "123" , }] , "type" : { "name" : null , "kind" : "NON_NULL" , "ofType" : { "name" : "SampleScalar" , "kind" : "SCALAR" , } , } , "isDeprecated" : false , "deprecationReason" : null , }))) ; }
};
}
