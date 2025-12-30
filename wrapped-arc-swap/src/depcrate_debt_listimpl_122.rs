// Generated macro for impl_122 (impl)
macro_rules! Depcrate_debt_listimpl_122 {
() => {
// Module: crate::debt::list
// Provides: {"impl_122"}
// Dependencies: {}
impl Drop for LocalNode { fn drop (& mut self) { if let Some (node) = self . node . get () { node . start_cooldown () ; } } }
};
}
