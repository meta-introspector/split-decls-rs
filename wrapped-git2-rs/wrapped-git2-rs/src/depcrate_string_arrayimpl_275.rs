// Generated macro for impl_275 (impl)
macro_rules! Depcrate_string_arrayimpl_275 {
() => {
// Module: crate::string_array
// Provides: {"impl_275"}
// Dependencies: {}
impl Drop for StringArray { fn drop (& mut self) { unsafe { raw :: git_strarray_free (& mut self . raw) } } }
};
}
