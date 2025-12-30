// Generated macro for impl_56 (impl)
macro_rules! Depcrate_object_urlimpl_56 {
() => {
// Module: crate::object_url
// Provides: {"impl_56"}
// Dependencies: {}
impl Drop for ObjectUrlAllocation { fn drop (& mut self) { web_sys :: Url :: revoke_object_url (& self . url) . unwrap_throw () ; } }
};
}
