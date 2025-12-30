// Generated macro for other_250 (other)
macro_rules! Depcrateother_250 {
() => {
// Module: crate
// Provides: {"other_250"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Stores local settings and submits SETTINGS frame.  The |iv| is the"] # [doc = " pointer to the array of :type:`nghttp2_settings_entry`.  The |niv|"] # [doc = " indicates the number of :type:`nghttp2_settings_entry`."] # [doc = ""] # [doc = " The |flags| is currently ignored and should be"] # [doc = " :enum:`NGHTTP2_FLAG_NONE`."] # [doc = ""] # [doc = " This function does not take ownership of the |iv|.  This function"] # [doc = " copies all the elements in the |iv|."] # [doc = ""] # [doc = " While updating individual stream's local window size, if the window"] # [doc = " size becomes strictly larger than NGHTTP2_MAX_WINDOW_SIZE,"] # [doc = " RST_STREAM is issued against such a stream."] # [doc = ""] # [doc = " SETTINGS with :enum:`NGHTTP2_FLAG_ACK` is automatically submitted"] # [doc = " by the library and application could not send it at its will."] # [doc = ""] # [doc = " This function returns 0 if it succeeds, or one of the following"] # [doc = " negative error codes:"] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_INVALID_ARGUMENT`"] # [doc = "     The |iv| contains invalid value (e.g., initial window size"] # [doc = "     strictly greater than (1 << 31) - 1."] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory."] pub fn nghttp2_submit_settings (session : * mut nghttp2_session , flags : u8 , iv : * const nghttp2_settings_entry , niv : usize ,) -> :: std :: os :: raw :: c_int ; }
};
}
