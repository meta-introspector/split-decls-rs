// Generated macro for impl_111 (impl)
macro_rules! Depcrate_rc_blockimpl_111 {
() => {
// Module: crate::rc_block
// Provides: {"impl_111"}
// Dependencies: {}
impl < F : ? Sized > Clone for RcBlock < F > { # [doc = " Increase the reference-count of the block."] # [doc (alias = "Block_copy")] # [doc (alias = "_Block_copy")] # [inline] fn clone (& self) -> Self { unsafe { Self :: copy (self . ptr . as_ptr ()) } . unwrap_or_else (| | rc_clone_fail ()) } }
};
}
