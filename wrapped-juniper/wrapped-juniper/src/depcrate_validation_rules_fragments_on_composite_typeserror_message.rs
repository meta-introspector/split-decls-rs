// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_fragments_on_composite_typeserror_message {
() => {
// Module: crate::validation::rules::fragments_on_composite_types
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (fragment_name : Option < & str > , on_type : & str) -> String { if let Some (name) = fragment_name { format ! (r#"Fragment "{name}" cannot condition non composite type "{on_type}"#) } else { format ! (r#"Fragment cannot condition on non composite type "{on_type}""#) } }
};
}
