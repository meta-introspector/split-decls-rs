// Generated macro for enum_deprecation_introspection (function)
macro_rules! Depcrate_executor_tests_introspection_enumsenum_deprecation_introspection {
() => {
// Module: crate::executor_tests::introspection::enums
// Provides: {"enum_deprecation_introspection"}
// Dependencies: {}
# [tokio :: test] async fn enum_deprecation_introspection () { let doc = r#"
    {
        __type(name: "EnumDeprecation") {
            name
            description
            enumValues(includeDeprecated: true) {
                name
                description
                isDeprecated
                deprecationReason
            }
        }
    }
    "# ; run_type_info_query (doc , | (type_info , values) | { assert_eq ! (type_info . get_field_value ("name") , Some (& graphql :: value ! ("EnumDeprecation")) ,) ; assert_eq ! (type_info . get_field_value ("description") , Some (& graphql :: value ! (null)) ,) ; assert_eq ! (values . len () , 2) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "FOO" , "description" : null , "isDeprecated" : true , "deprecationReason" : "Please don't use FOO any more" , }))) ; assert ! (values . contains (& graphql :: value ! ({ "name" : "BAR" , "description" : "The BAR value" , "isDeprecated" : true , "deprecationReason" : "Please don't use BAR any more" , }))) ; }) . await ; }
};
}
