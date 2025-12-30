// Generated macro for EnumDeprecation (enum)
macro_rules! Depcrate_executor_tests_introspection_enumsEnumDeprecation {
() => {
// Module: crate::executor_tests::introspection::enums
// Provides: {"EnumDeprecation"}
// Dependencies: {}
# [derive (GraphQLEnum)] enum EnumDeprecation { # [graphql (deprecated = "Please don't use FOO any more")] Foo , # [graphql (description = "The BAR value" , deprecated = "Please don't use BAR any more")] Bar , }
};
}
