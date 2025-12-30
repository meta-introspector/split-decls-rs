// Generated macro for align_u64 (function)
macro_rules! Depcrate_write_utilalign_u64 {
() => {
// Module: crate::write::util
// Provides: {"align_u64"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn align_u64 (offset : u64 , size : u64) -> u64 { (offset + (size - 1)) & ! (size - 1) }
};
}
