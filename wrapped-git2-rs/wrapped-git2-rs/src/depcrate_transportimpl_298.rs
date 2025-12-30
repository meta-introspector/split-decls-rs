// Generated macro for impl_298 (impl)
macro_rules! Depcrate_transportimpl_298 {
() => {
// Module: crate::transport
// Provides: {"impl_298"}
// Dependencies: {}
impl Drop for Transport { fn drop (& mut self) { if self . owned { unsafe { (* self . raw) . free . unwrap () (self . raw) } } } }
};
}
