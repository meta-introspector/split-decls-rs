// Generated macro for field_description_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_input_objectfield_description_introspection {
() => {
// Module: crate::executor_tests::introspection::input_object
// Provides: {"field_description_introspection"}
// Dependencies: {}
# [tokio :: test] async fn field_description_introspection () { let doc = r#"{
        __type(name: "FieldDescription") {
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
    }"# ; run_type_info_query (doc , | type_info , fields | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("FieldDescription")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (fields . len () , 2) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "fieldOne" , "description" : "The first field" , "type" : { "ofType" : { "name" : "String" } , } , "defaultValue" : null , }))) ; assert ! (fields . contains (& graphql :: value ! ({ "name" : "fieldTwo" , "description" : "The second field" , "type" : { "ofType" : { "name" : "String" } , } , "defaultValue" : null , }))) ; }) . await ; }
};
}
