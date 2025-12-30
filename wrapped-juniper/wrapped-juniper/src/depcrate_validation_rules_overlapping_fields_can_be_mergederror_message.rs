// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergederror_message {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (reason_name : & str , reason : & ConflictReasonMessage) -> String { let suffix = "Use different aliases on the fields to fetch both if this was intentional" ; format ! (r#"Fields "{reason_name}" conflict because {}. {suffix}"# , format_reason (reason) ,) }
};
}
