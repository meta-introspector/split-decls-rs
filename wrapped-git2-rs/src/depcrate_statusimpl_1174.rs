// Generated macro for impl_1174 (impl)
macro_rules! Depcrate_statusimpl_1174 {
() => {
// Module: crate::status
// Provides: {"impl_1174"}
// Dependencies: {}
impl < 'repo > Drop for Statuses < 'repo > { fn drop (& mut self) { unsafe { raw :: git_status_list_free (self . raw) ; } } }
};
}
