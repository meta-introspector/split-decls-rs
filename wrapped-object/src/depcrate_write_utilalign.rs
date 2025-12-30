// Generated macro for align (function)
macro_rules! Depcrate_write_utilalign {
() => {
// Module: crate::write::util
// Provides: {"align"}
// Dependencies: {}
pub (crate) fn align (offset : usize , size : usize) -> usize { (offset + (size - 1)) & ! (size - 1) }
};
}
