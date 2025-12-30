// Generated macro for mapping_version (function)
macro_rules! Depcrate_coverageinfo_llvm_covmapping_version {
() => {
// Module: crate::coverageinfo::llvm_cov
// Provides: {"mapping_version"}
// Dependencies: {}
# [doc = " Returns LLVM's `coverage::CovMapVersion::CurrentVersion` (CoverageMapping.h)"] # [doc = " as a raw numeric value. For historical reasons, the numeric value is 1 less"] # [doc = " than the number in the version's name, so `Version7` is actually `6u32`."] pub (crate) fn mapping_version () -> u32 { unsafe { llvm :: LLVMRustCoverageMappingVersion () } }
};
}
