// Generated macro for quiche_h3_event_for_each_header (function)
macro_rules! Depcrate_h3_ffiquiche_h3_event_for_each_header {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_event_for_each_header"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_event_for_each_header (ev : & h3 :: Event , cb : extern "C" fn (name : * const u8 , name_len : size_t , value : * const u8 , value_len : size_t , argp : * mut c_void ,) -> c_int , argp : * mut c_void ,) -> c_int { match ev { h3 :: Event :: Headers { list , .. } => for h in list { let rc = cb (h . name () . as_ptr () , h . name () . len () , h . value () . as_ptr () , h . value () . len () , argp ,) ; if rc != 0 { return rc ; } } , _ => unreachable ! () , } 0 }
};
}
