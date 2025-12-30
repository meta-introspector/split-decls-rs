// Generated macro for align_up (function)
macro_rules! Depcrate_typesalign_up {
() => {
// Module: crate::types
// Provides: {"align_up"}
// Dependencies: {}
pub fn align_up (pos : usize , align : usize) -> usize { (pos + align - 1) & ! (align - 1) }
};
}
