// Generated macro for impl_63 (impl)
macro_rules! Depcrate_easy_formimpl_63 {
() => {
// Module: crate::easy::form
// Provides: {"impl_63"}
// Dependencies: {}
impl Drop for Form { fn drop (& mut self) { unsafe { curl_sys :: curl_formfree (self . head) ; } } }
};
}
