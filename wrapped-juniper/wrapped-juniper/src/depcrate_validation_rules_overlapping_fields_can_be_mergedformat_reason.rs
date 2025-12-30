// Generated macro for format_reason (function)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedformat_reason {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"format_reason"}
// Dependencies: {}
fn format_reason (reason : & ConflictReasonMessage) -> impl Display { match reason { ConflictReasonMessage :: Message (name) => Either :: Left (name) , ConflictReasonMessage :: Nested (nested) => Either :: Right (nested . iter () . format_with (" and " , | ConflictReason (name , subreason) , f | { f (& format_args ! (r#"subfields "{name}" conflict because {}"# , format_reason (subreason) ,)) } ,)) , } }
};
}
