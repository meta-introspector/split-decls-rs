// Generated macro for RareByteOffsets (struct)
macro_rules! Depcrate_util_prefilterRareByteOffsets {
() => {
// Module: crate::util::prefilter
// Provides: {"RareByteOffsets"}
// Dependencies: {}
# [doc = " A set of byte offsets, keyed by byte."] # [derive (Clone , Copy)] struct RareByteOffsets { # [doc = " Each entry corresponds to the maximum offset of the corresponding"] # [doc = " byte across all patterns seen."] set : [RareByteOffset ; 256] , }
};
}
