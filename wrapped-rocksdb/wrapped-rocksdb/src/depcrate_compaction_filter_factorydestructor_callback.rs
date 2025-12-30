// Generated macro for destructor_callback (function)
macro_rules! Depcrate_compaction_filter_factorydestructor_callback {
() => {
// Module: crate::compaction_filter_factory
// Provides: {"destructor_callback"}
// Dependencies: {}
pub unsafe extern "C" fn destructor_callback < F > (raw_self : * mut c_void) where F : CompactionFilterFactory , { drop (unsafe { Box :: from_raw (raw_self as * mut F) }) ; }
};
}
