// Generated macro for named_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_input_objectnamed_introspection {
() => {
// Module: crate::executor_tests::introspection::input_object
// Provides: {"named_introspection"}
// Dependencies: {}
# [tokio :: test] async fn named_introspection () { let doc = r#"{
        __type(name: "ANamedInputObject") {
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
    }"# ; run_type_info_query (doc , | type_info , fields | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("ANamedInputObject"))) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! (null))) ; assert_eq ! (fields . len () , 1) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "fieldOne" , "description" : null , "type" : { "ofType" : { "name" : "String" } , } , "defaultValue" : null , }))) ; }) . await ; }
};
}
