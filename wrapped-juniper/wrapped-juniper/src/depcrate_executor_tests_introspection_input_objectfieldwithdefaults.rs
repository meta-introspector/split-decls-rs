// Generated macro for FieldWithDefaults (struct)
macro_rules! Depcrate_executor_tests_introspection_input_objectFieldWithDefaults {
() => {
// Module: crate::executor_tests::introspection::input_object
// Provides: {"FieldWithDefaults"}
// Dependencies: {}
# [derive (GraphQLInputObject , Debug)] struct FieldWithDefaults { # [graphql (default = 123)] field_one : i32 , # [graphql (default = 456 , description = "The second field")] field_two : i32 , }
};
}
