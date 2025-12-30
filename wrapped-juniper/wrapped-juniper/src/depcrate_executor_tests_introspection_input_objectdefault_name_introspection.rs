// Generated macro for default_name_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_input_objectdefault_name_introspection {
() => {
// Module: crate::executor_tests::introspection::input_object
// Provides: {"default_name_introspection"}
// Dependencies: {}
# [tokio :: test] async fn default_name_introspection () { let doc = r#"{
        __type(name: "DefaultName") {
            name
            description
            inputFields {
                name
                description
                type {
                    ofType {
                        name
                    }
                }
                defaultValue
            }
        }
    }"# ; run_type_info_query (doc , | type_info , fields | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("DefaultName")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (fields . len () , 2) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "fieldOne" , "description" : null , "type" : { "ofType" : { "name" : "String" } , } , "defaultValue" : null , }))) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "fieldTwo" , "description" : null , "type" : { "ofType" : { "name" : "String" } , } , "defaultValue" : null , }))) ; }) . await ; }
};
}
