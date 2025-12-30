// Generated macro for impl_234 (impl)
macro_rules! Depcrate_vec_set_len_on_dropimpl_234 {
() => {
// Module: crate::vec::set_len_on_drop
// Provides: {"impl_234"}
// Dependencies: {}
impl Drop for SetLenOnDrop < '_ > { # [inline (always)] fn drop (& mut self) { * self . len = self . local_len ; } }
};
}
