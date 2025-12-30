// Generated macro for impl_77 (impl)
macro_rules! Depcrate_mockimpl_77 {
() => {
// Module: crate::mock
// Provides: {"impl_77"}
// Dependencies: {}
impl Drop for Mock { fn drop (& mut self) { let mut me = self . pipe . inner . lock () . unwrap () ; me . closed = true ; if let Some (task) = me . tx_task . take () { task . wake () ; } } }
};
}
