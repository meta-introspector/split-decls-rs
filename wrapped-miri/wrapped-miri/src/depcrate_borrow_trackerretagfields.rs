// Generated macro for RetagFields (enum)
macro_rules! Depcrate_borrow_trackerRetagFields {
() => {
// Module: crate::borrow_tracker
// Provides: {"RetagFields"}
// Dependencies: {}
# [doc = " Policy on whether to recurse into fields to retag"] # [derive (Copy , Clone , Debug)] pub enum RetagFields { # [doc = " Don't retag any fields."] No , # [doc = " Retag all fields."] Yes , # [doc = " Only retag fields of types with Scalar and ScalarPair layout,"] # [doc = " to match the LLVM `noalias` we generate."] OnlyScalar , }
};
}
