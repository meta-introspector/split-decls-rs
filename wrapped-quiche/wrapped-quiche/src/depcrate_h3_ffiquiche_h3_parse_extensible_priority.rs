// Generated macro for quiche_h3_parse_extensible_priority (function)
macro_rules! Depcrate_h3_ffiquiche_h3_parse_extensible_priority {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_parse_extensible_priority"}
// Dependencies: {}
# [no_mangle] # [cfg (feature = "sfv")] pub extern "C" fn quiche_h3_parse_extensible_priority (priority : * const u8 , priority_len : size_t , parsed : & mut Priority ,) -> c_int { let priority = unsafe { slice :: from_raw_parts (priority , priority_len) } ; match Priority :: try_from (priority) { Ok (v) => { parsed . urgency = v . urgency ; parsed . incremental = v . incremental ; 0 } , Err (e) => e . to_c () as c_int , } }
};
}
