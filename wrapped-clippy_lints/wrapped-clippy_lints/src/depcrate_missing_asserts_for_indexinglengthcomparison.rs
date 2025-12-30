// Generated macro for LengthComparison (enum)
macro_rules! Depcrate_missing_asserts_for_indexingLengthComparison {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"LengthComparison"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] enum LengthComparison { # [doc = " `v.len() < 5`"] LengthLessThanInt , # [doc = " `5 < v.len()`"] IntLessThanLength , # [doc = " `v.len() <= 5`"] LengthLessThanOrEqualInt , # [doc = " `5 <= v.len()`"] IntLessThanOrEqualLength , # [doc = " `5 == v.len()`"] # [doc = " `v.len() == 5`"] LengthEqualInt , }
};
}
