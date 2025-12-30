// Generated macro for align_u32 (function)
macro_rules! Depcrate_write_utilalign_u32 {
() => {
// Module: crate::write::util
// Provides: {"align_u32"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn align_u32 (offset : u32 , size : u32) -> u32 { (offset + (size - 1)) & ! (size - 1) }
};
}
