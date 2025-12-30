// Generated macro for impl_37 (impl)
macro_rules! Depcrate_msgimpl_37 {
() => {
// Module: crate::msg
// Provides: {"impl_37"}
// Dependencies: {}
impl Drop for CommandMessages { fn drop (& mut self) { if ! self . 0 . done { let _ = self . 0 . child . wait () ; } } }
};
}
