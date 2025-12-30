// Generated macro for impl_166 (impl)
macro_rules! Depcrate_easy_listimpl_166 {
() => {
// Module: crate::easy::list
// Provides: {"impl_166"}
// Dependencies: {}
impl Drop for List { fn drop (& mut self) { unsafe { curl_sys :: curl_slist_free_all (self . raw) } } }
};
}
