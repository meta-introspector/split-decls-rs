// Generated macro for BranchRegion (struct)
macro_rules! Depcrate_coverageinfo_ffiBranchRegion {
() => {
// Module: crate::coverageinfo::ffi
// Provides: {"BranchRegion"}
// Dependencies: {}
# [doc = " Must match the layout of `LLVMRustCoverageBranchRegion`."] # [derive (Clone , Debug)] # [repr (C)] pub (crate) struct BranchRegion { pub (crate) cov_span : CoverageSpan , pub (crate) true_counter : Counter , pub (crate) false_counter : Counter , }
};
}
