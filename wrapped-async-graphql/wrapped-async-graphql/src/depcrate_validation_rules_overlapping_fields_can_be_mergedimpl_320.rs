// Generated macro for impl_320 (impl)
macro_rules! Depcrate_validation_rules_overlapping_fields_can_be_mergedimpl_320 {
() => {
// Module: crate::validation::rules::overlapping_fields_can_be_merged
// Provides: {"impl_320"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for OverlappingFieldsCanBeMerged { fn enter_selection_set (& mut self , ctx : & mut VisitorContext < 'a > , selection_set : & 'a Positioned < SelectionSet > ,) { let mut find_conflicts = FindConflicts { outputs : Default :: default () , visited : Default :: default () , ctx , } ; find_conflicts . find (None , selection_set) ; } }
};
}
