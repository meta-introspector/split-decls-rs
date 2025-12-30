// Generated macro for quiche_h3_take_last_priority_update (function)
macro_rules! Depcrate_h3_ffiquiche_h3_take_last_priority_update {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_take_last_priority_update"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_take_last_priority_update (conn : & mut h3 :: Connection , prioritized_element_id : u64 , cb : extern "C" fn (priority_field_value : * const u8 , priority_field_value_len : size_t , argp : * mut c_void ,) -> c_int , argp : * mut c_void ,) -> c_int { match conn . take_last_priority_update (prioritized_element_id) { Ok (priority) => { let rc = cb (priority . as_ptr () , priority . len () , argp) ; if rc != 0 { return rc ; } 0 } , Err (e) => e . to_c () as c_int , } }
};
}
