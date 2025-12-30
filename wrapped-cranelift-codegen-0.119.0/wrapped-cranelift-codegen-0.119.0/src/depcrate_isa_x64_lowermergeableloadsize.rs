// Generated macro for MergeableLoadSize (enum)
macro_rules! Depcrate_isa_x64_lowerMergeableLoadSize {
() => {
// Module: crate::isa::x64::lower
// Provides: {"MergeableLoadSize"}
// Dependencies: {}
enum MergeableLoadSize { # [doc = " The load size performed by a sinkable load merging operation is"] # [doc = " precisely the size necessary for the type in question."] Exact , # [doc = " Narrower-than-32-bit values are handled by ALU insts that are at least"] # [doc = " 32 bits wide, which is normally OK as we ignore upper buts; but, if we"] # [doc = " generate, e.g., a direct-from-memory 32-bit add for a byte value and"] # [doc = " the byte is the last byte in a page, the extra data that we load is"] # [doc = " incorrectly accessed. So we only allow loads to merge for"] # [doc = " 32-bit-and-above widths."] Min32 , }
};
}
