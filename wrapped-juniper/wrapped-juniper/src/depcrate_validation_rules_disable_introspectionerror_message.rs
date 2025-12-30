// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_disable_introspectionerror_message {
() => {
// Module: crate::validation::rules::disable_introspection
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (field_name : & str) -> String { format ! ("GraphQL introspection is not allowed, but the operation contained `{field_name}`") }
};
}
