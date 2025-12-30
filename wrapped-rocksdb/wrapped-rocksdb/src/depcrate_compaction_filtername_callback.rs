// Generated macro for name_callback (function)
macro_rules! Depcrate_compaction_filtername_callback {
() => {
// Module: crate::compaction_filter
// Provides: {"name_callback"}
// Dependencies: {}
pub unsafe extern "C" fn name_callback < F > (raw_cb : * mut c_void) -> * const c_char where F : CompactionFilter , { let cb = unsafe { & * (raw_cb as * mut F) } ; cb . name () . as_ptr () }
};
}
