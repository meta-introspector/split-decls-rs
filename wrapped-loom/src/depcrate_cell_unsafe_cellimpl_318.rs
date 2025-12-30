// Generated macro for impl_318 (impl)
macro_rules! Depcrate_cell_unsafe_cellimpl_318 {
() => {
// Module: crate::cell::unsafe_cell
// Provides: {"impl_318"}
// Dependencies: {}
impl < T > From < T > for UnsafeCell < T > { fn from (src : T) -> UnsafeCell < T > { UnsafeCell :: new (src) } }
};
}
