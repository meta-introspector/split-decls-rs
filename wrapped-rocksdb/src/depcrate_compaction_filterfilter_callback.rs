// Generated macro for filter_callback (function)
macro_rules! Depcrate_compaction_filterfilter_callback {
() => {
// Module: crate::compaction_filter
// Provides: {"filter_callback"}
// Dependencies: {}
pub unsafe extern "C" fn filter_callback < F > (raw_cb : * mut c_void , level : c_int , raw_key : * const c_char , key_length : size_t , existing_value : * const c_char , value_length : size_t , new_value : * mut * mut c_char , new_value_length : * mut size_t , value_changed : * mut c_uchar ,) -> c_uchar where F : CompactionFilter , { use self :: Decision :: { Change , Keep , Remove } ; let cb = unsafe { & mut * (raw_cb as * mut F) } ; let key = unsafe { slice :: from_raw_parts (raw_key as * const u8 , key_length) } ; let oldval = unsafe { slice :: from_raw_parts (existing_value as * const u8 , value_length) } ; let result = cb . filter (level as u32 , key , oldval) ; match result { Keep => 0 , Remove => 1 , Change (newval) => { unsafe { * new_value = newval . as_ptr () as * mut c_char } ; unsafe { * new_value_length = newval . len () as size_t } ; unsafe { * value_changed = 1_u8 } ; 0 } } }
};
}
