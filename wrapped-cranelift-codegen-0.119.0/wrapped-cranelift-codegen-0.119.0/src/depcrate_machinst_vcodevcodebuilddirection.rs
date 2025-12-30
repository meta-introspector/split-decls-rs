// Generated macro for VCodeBuildDirection (enum)
macro_rules! Depcrate_machinst_vcodeVCodeBuildDirection {
() => {
// Module: crate::machinst::vcode
// Provides: {"VCodeBuildDirection"}
// Dependencies: {}
# [doc = " Direction in which a VCodeBuilder builds VCode."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum VCodeBuildDirection { # [doc = " Backward-build pass: we expect the producer to call `emit()`"] # [doc = " with instructions in reverse program order within each block."] Backward , }
};
}
