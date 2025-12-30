// Generated macro for fuzzy_metadata_eq (function)
macro_rules! Depcrate_core_builder_testsfuzzy_metadata_eq {
() => {
// Module: crate::core::builder::tests
// Provides: {"fuzzy_metadata_eq"}
// Dependencies: {}
fn fuzzy_metadata_eq (executed : & StepMetadata , to_match : & StepMetadata) -> bool { let StepMetadata { name , kind , target , built_by : _ , stage : _ , metadata } = executed ; * name == to_match . name && * kind == to_match . kind && * target == to_match . target }
};
}
