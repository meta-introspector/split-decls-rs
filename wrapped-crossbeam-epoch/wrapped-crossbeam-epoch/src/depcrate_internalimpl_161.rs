// Generated macro for impl_161 (impl)
macro_rules! Depcrate_internalimpl_161 {
() => {
// Module: crate::internal
// Provides: {"impl_161"}
// Dependencies: {}
impl IsElement < Self > for Local { fn entry_of (local : & Self) -> & Entry { unsafe { let entry_ptr = (local as * const Self) . cast :: < Entry > () ; & * entry_ptr } } unsafe fn element_of (entry : & Entry) -> & Self { unsafe { let local_ptr = (entry as * const Entry) . cast :: < Self > () ; & * local_ptr } } unsafe fn finalize (entry : & Entry , guard : & Guard) { unsafe { guard . defer_destroy (Shared :: from (Self :: element_of (entry) as * const _)) } } }
};
}
