// Generated macro for enum_value_description_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_enumsenum_value_description_introspection {
() => {
// Module: crate::executor_tests::introspection::enums
// Provides: {"enum_value_description_introspection"}
// Dependencies: {}
# [tokio :: test] async fn enum_value_description_introspection () { let doc = r#"
    {
        __type(name: "EnumValueDescription") {
            name
            description
            enumValues {
                name
                description
                isDeprecated
                deprecationReason
            }
        }
    }
    "# ; run_type_info_query (doc , | (type_info , values) | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("EnumValueDescription")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (values . len () , 2) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "FOO" , "description" : "The FOO value" , "isDeprecated" : false , "deprecationReason" : null , }))) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "BAR" , "description" : "The BAR value" , "isDeprecated" : false , "deprecationReason" : null , }))) ; }) . await ; }
};
}
