// Generated macro for MappingKind (enum)
macro_rules! Depcrate_covfunMappingKind {
() => {
// Module: crate::covfun
// Provides: {"MappingKind"}
// Dependencies: {}
# [derive (Debug)] enum MappingKind { Code (CovTerm) , Gap (CovTerm) , Expansion (# [allow (dead_code)] u32) , Skip , Branch { r#true : CovTerm , r#false : CovTerm , } , MCDCBranch { r#true : CovTerm , r#false : CovTerm , # [allow (dead_code)] condition_id : u32 , # [allow (dead_code)] true_next_id : u32 , # [allow (dead_code)] false_next_id : u32 , } , MCDCDecision { # [allow (dead_code)] bitmap_idx : u32 , # [allow (dead_code)] conditions_num : u32 , } , }
};
}
