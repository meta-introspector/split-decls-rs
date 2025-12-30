// Generated macro for CovmapVersion (enum)
macro_rules! Depcrate_coverageinfo_mapgenCovmapVersion {
() => {
// Module: crate::coverageinfo::mapgen
// Provides: {"CovmapVersion"}
// Dependencies: {}
# [doc = " Version number that will be included the `__llvm_covmap` section header."] # [doc = " Corresponds to LLVM's `llvm::coverage::CovMapVersion` (in `CoverageMapping.h`),"] # [doc = " or at least the subset that we know and care about."] # [doc = ""] # [doc = " Note that version `n` is encoded as `(n-1)`."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , TryFromU32)] enum CovmapVersion { # [doc = " Used by LLVM 18 onwards."] Version7 = 6 , }
};
}
