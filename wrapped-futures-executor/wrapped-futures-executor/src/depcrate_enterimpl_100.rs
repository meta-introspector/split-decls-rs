// Generated macro for impl_100 (impl)
macro_rules! Depcrate_enterimpl_100 {
() => {
// Module: crate::enter
// Provides: {"impl_100"}
// Dependencies: {}
impl Drop for Enter { fn drop (& mut self) { ENTERED . with (| c | { assert ! (c . get ()) ; c . set (false) ; }) ; } }
};
}
