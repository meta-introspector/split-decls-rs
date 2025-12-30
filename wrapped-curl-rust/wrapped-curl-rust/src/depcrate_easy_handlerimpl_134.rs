// Generated macro for impl_134 (impl)
macro_rules! Depcrate_easy_handlerimpl_134 {
() => {
// Module: crate::easy::handler
// Provides: {"impl_134"}
// Dependencies: {}
impl < H > Drop for Easy2 < H > { fn drop (& mut self) { unsafe { curl_sys :: curl_easy_cleanup (self . inner . handle) ; } } }
};
}
