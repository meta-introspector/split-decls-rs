// Generated macro for default_name_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_enumsdefault_name_introspection {
() => {
// Module: crate::executor_tests::introspection::enums
// Provides: {"default_name_introspection"}
// Dependencies: {}
# [tokio :: test] async fn default_name_introspection () { let doc = r#"
    {
        __type(name: "DefaultName") {
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
    "# ; run_type_info_query (doc , | (type_info , values) | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("DefaultName")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (values . len () , 2) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "FOO" , "description" : null , "isDeprecated" : false , "deprecationReason" : null , }))) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "BAR" , "description" : null , "isDeprecated" : false , "deprecationReason" : null , }))) ; }) . await ; }
};
}
