// Generated macro for factory (function)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedfactory {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"factory"}
// Dependencies: {}
pub fn factory < 'a , S : Debug > () -> OverlappingFieldsCanBeMerged < 'a , S > { OverlappingFieldsCanBeMerged { named_fragments : HashMap :: new () , compared_fragments : RefCell :: new (PairSet :: new ()) , } }
};
}
