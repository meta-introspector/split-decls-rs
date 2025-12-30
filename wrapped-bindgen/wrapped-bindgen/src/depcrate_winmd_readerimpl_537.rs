// Generated macro for impl_537 (impl)
macro_rules! Depcrate_winmd_readerimpl_537 {
() => {
// Module: crate::winmd::reader
// Provides: {"impl_537"}
// Dependencies: {}
impl Drop for Reader { fn drop (& mut self) { for file in & self . 1 { unsafe { _ = Box :: from_raw (* file) ; } } } }
};
}
