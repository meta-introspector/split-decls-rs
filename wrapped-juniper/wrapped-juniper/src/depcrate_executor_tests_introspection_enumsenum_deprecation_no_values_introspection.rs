// Generated macro for enum_deprecation_no_values_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_enumsenum_deprecation_no_values_introspection {
() => {
// Module: crate::executor_tests::introspection::enums
// Provides: {"enum_deprecation_no_values_introspection"}
// Dependencies: {}
# [tokio :: test] async fn enum_deprecation_no_values_introspection () { let doc = r#"
    {
        __type(name: "EnumDeprecation") {
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
    "# ; run_type_info_query (doc , | (type_info , values) | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("EnumDeprecation")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (values . len () , 0) ; }) . await ; }
};
}
