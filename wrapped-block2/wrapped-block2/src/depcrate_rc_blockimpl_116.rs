// Generated macro for impl_116 (impl)
macro_rules! Depcrate_rc_blockimpl_116 {
() => {
// Module: crate::rc_block
// Provides: {"impl_116"}
// Dependencies: {}
impl < F : ? Sized > Drop for RcBlock < F > { # [doc = " Release the block, decreasing the reference-count by 1."] # [doc = ""] # [doc = " The `Drop` method of the underlying closure will be called once the"] # [doc = " reference-count reaches zero."] # [doc (alias = "Block_release")] # [doc (alias = "_Block_release")] # [inline] fn drop (& mut self) { unsafe { ffi :: _Block_release (self . ptr . as_ptr () . cast ()) } ; } }
};
}
