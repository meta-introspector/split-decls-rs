// Generated macro for align (function)
macro_rules! Depcrate_read_utilalign {
() => {
// Module: crate::read::util
// Provides: {"align"}
// Dependencies: {}
# [allow (dead_code)] # [inline] pub (crate) fn align (offset : usize , size : usize) -> usize { (offset + (size - 1)) & ! (size - 1) }
};
}
