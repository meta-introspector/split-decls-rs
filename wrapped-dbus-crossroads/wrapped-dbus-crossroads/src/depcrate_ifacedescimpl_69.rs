// Generated macro for impl_69 (impl)
macro_rules! Depcrate_ifacedescimpl_69 {
() => {
// Module: crate::ifacedesc
// Provides: {"impl_69"}
// Dependencies: {}
impl < T , A > Drop for PropBuilder < '_ , T , A > { fn drop (& mut self) { assert ! (self . desc . get_cb . is_some () || self . desc . set_cb . is_some ()) ; } }
};
}
