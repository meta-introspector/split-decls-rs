// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl From < gccrs_ffi :: FactsView > for AllFacts < GccrsFacts > { fn from (input : gccrs_ffi :: FactsView) -> Self { AllFacts :: < GccrsFacts > { loan_issued_at : input . loan_issued_at . into () , universal_region : input . universal_region . into () , cfg_edge : input . cfg_edge . into () , loan_killed_at : input . loan_killed_at . into () , subset_base : input . subset_base . into () , loan_invalidated_at : input . loan_invalidated_at . into () , var_used_at : input . var_used_at . into () , var_defined_at : input . var_defined_at . into () , var_dropped_at : input . var_dropped_at . into () , use_of_var_derefs_origin : input . use_of_var_derefs_origin . into () , drop_of_var_derefs_origin : input . drop_of_var_derefs_origin . into () , child_path : input . child_path . into () , path_is_var : input . path_is_var . into () , path_assigned_at_base : input . path_assigned_at_base . into () , path_moved_at_base : input . path_moved_at_base . into () , path_accessed_at_base : input . path_accessed_at_base . into () , known_placeholder_subset : input . known_placeholder_subset . into () , placeholder : input . placeholder . into () , } } }
};
}
