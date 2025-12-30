// Generated macro for enum_description_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_enumsenum_description_introspection {
() => {
// Module: crate::executor_tests::introspection::enums
// Provides: {"enum_description_introspection"}
// Dependencies: {}
# [tokio :: test] async fn enum_description_introspection () { let doc = r#"
    {
        __type(name: "EnumDescription") {
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
    "# ; run_type_info_query (doc , | (type_info , values) | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("EnumDescription")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! ("A description of the enum itself")) ,) ; assert_eq ! (values . len () , 2) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "FOO" , "description" : null , "isDeprecated" : false , "deprecationReason" : null , }))) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "BAR" , "description" : null , "isDeprecated" : false , "deprecationReason" : null , }))) ; }) . await ; }
};
}
