// Generated macro for misplaced_error_message (function)
macro_rules! Depcrate_validation_rules_known_directivesmisplaced_error_message {
() => {
// Module: crate::validation::rules::known_directives
// Provides: {"misplaced_error_message"}
// Dependencies: {}
fn misplaced_error_message (directive_name : & str , location : & DirectiveLocation) -> String { format ! (r#"Directive "{directive_name}" may not be used on {location}"#) }
};
}
