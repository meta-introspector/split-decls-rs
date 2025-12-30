// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl Drop for HelperThread { fn drop (& mut self) { self . state . lock () . producer_done = true ; self . state . cvar . notify_one () ; self . inner . take () . unwrap () . join () ; } }
};
}
