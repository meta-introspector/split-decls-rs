// Generated macro for field_with_defaults_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_input_objectfield_with_defaults_introspection {
() => {
// Module: crate::executor_tests::introspection::input_object
// Provides: {"field_with_defaults_introspection"}
// Dependencies: {}
# [tokio :: test] async fn field_with_defaults_introspection () { let doc = r#"{
        __type(name: "FieldWithDefaults") {
            name
            inputFields {
                name
                type {
                    name
                    ofType {
                        name
                    }
                }
                defaultValue
            }
        }
    }"# ; run_type_info_query (doc , | type_info , fields | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("FieldWithDefaults")) ,) ; assert_eq ! (fields . len () , 2) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "fieldOne" , "type" : { "name" : null , "ofType" : { "name" : "Int" } } , "defaultValue" : "123" , }))) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "fieldTwo" , "type" : { "name" : null , "ofType" : { "name" : "Int" } } , "defaultValue" : "456" , }))) ; }) . await ; }
};
}
