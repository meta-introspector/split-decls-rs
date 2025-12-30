// Generated macro for impl_393 (impl)
macro_rules! Depcrate_bufimpl_393 {
() => {
// Module: crate::buf
// Provides: {"impl_393"}
// Dependencies: {}
impl Drop for Buf { fn drop (& mut self) { unsafe { raw :: git_buf_dispose (& mut self . raw) } } }
};
}
