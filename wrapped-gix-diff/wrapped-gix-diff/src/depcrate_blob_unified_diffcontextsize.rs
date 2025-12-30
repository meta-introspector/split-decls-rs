// Generated macro for ContextSize (struct)
macro_rules! Depcrate_blob_unified_diffContextSize {
() => {
// Module: crate::blob::unified_diff
// Provides: {"ContextSize"}
// Dependencies: {}
# [doc = " Defines the size of the context printed before and after each change."] # [doc = ""] # [doc = " Similar to the `-U` option in git diff or gnu-diff. If the context overlaps"] # [doc = " with previous or next change, the context gets reduced accordingly."] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq , Ord , PartialOrd)] pub struct ContextSize { # [doc = " Defines the size of the context printed before and after each change."] symmetrical : u32 , }
};
}
