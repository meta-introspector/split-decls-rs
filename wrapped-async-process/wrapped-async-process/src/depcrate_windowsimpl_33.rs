// Generated macro for impl_33 (impl)
macro_rules! Depcrate_windowsimpl_33 {
() => {
// Module: crate::windows
// Provides: {"impl_33"}
// Dependencies: {}
impl AsRawHandle for Child { fn as_raw_handle (& self) -> RawHandle { self . child . lock () . unwrap () . get_mut () . as_raw_handle () } }
};
}
