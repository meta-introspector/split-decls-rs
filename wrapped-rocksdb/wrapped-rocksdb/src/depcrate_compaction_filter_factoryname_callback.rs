// Generated macro for name_callback (function)
macro_rules! Depcrate_compaction_filter_factoryname_callback {
() => {
// Module: crate::compaction_filter_factory
// Provides: {"name_callback"}
// Dependencies: {}
pub unsafe extern "C" fn name_callback < F > (raw_self : * mut c_void) -> * const c_char where F : CompactionFilterFactory , { let self_ = unsafe { & * (raw_self . cast_const () as * const F) } ; self_ . name () . as_ptr () }
};
}
