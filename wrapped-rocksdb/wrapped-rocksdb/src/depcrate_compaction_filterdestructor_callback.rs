// Generated macro for destructor_callback (function)
macro_rules! Depcrate_compaction_filterdestructor_callback {
() => {
// Module: crate::compaction_filter
// Provides: {"destructor_callback"}
// Dependencies: {}
pub unsafe extern "C" fn destructor_callback < F > (raw_cb : * mut c_void) where F : CompactionFilter , { drop (unsafe { Box :: from_raw (raw_cb as * mut F) }) ; }
};
}
