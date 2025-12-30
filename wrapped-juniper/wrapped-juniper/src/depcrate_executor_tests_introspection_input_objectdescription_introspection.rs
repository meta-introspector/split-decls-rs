// Generated macro for description_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_input_objectdescription_introspection {
() => {
// Module: crate::executor_tests::introspection::input_object
// Provides: {"description_introspection"}
// Dependencies: {}
# [tokio :: test] async fn description_introspection () { let doc = r#"{
        __type(name: "Description") {
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
    }"# ; run_type_info_query (doc , | type_info , fields | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("Description")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! ("Description for the input object")) ,) ; assert_eq ! (fields . len () , 1) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "fieldOne" , "description" : null , "type" : { "ofType" : { "name" : "String" } , } , "defaultValue" : null , }))) ; }) . await ; }
};
}
