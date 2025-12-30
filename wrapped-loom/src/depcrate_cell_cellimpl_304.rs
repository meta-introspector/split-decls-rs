// Generated macro for impl_304 (impl)
macro_rules! Depcrate_cell_cellimpl_304 {
() => {
// Module: crate::cell::cell
// Provides: {"impl_304"}
// Dependencies: {}
impl < T : Copy > Clone for Cell < T > { # [track_caller] fn clone (& self) -> Cell < T > { Cell :: new (self . get ()) } }
};
}
