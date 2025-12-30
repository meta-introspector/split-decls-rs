// Generated macro for impl_317 (impl)
macro_rules! Depcrate_cell_unsafe_cellimpl_317 {
() => {
// Module: crate::cell::unsafe_cell
// Provides: {"impl_317"}
// Dependencies: {}
impl < T : Default > Default for UnsafeCell < T > { fn default () -> UnsafeCell < T > { UnsafeCell :: new (Default :: default ()) } }
};
}
