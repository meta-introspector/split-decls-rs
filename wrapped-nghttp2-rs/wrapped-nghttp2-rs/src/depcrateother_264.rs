// Generated macro for other_264 (other)
macro_rules! Depcrateother_264 {
() => {
// Module: crate
// Provides: {"other_264"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Submits ORIGIN frame."] # [doc = ""] # [doc = " ORIGIN frame is a non-critical extension to HTTP/2 and defined by"] # [doc = " `RFC 8336 <https://tools.ietf.org/html/rfc8336>`_."] # [doc = ""] # [doc = " The |flags| is currently ignored and should be"] # [doc = " :enum:`NGHTTP2_FLAG_NONE`."] # [doc = ""] # [doc = " The |ov| points to the array of origins.  The |nov| specifies the"] # [doc = " number of origins included in |ov|.  This function creates copies"] # [doc = " of all elements in |ov|."] # [doc = ""] # [doc = " The ORIGIN frame is only usable by a server.  If this function is"] # [doc = " invoked with client side session, this function returns"] # [doc = " :enum:`NGHTTP2_ERR_INVALID_STATE`."] # [doc = ""] # [doc = " :enum:`NGHTTP2_ERR_NOMEM`"] # [doc = "     Out of memory"] # [doc = " :enum:`NGHTTP2_ERR_INVALID_STATE`"] # [doc = "     The function is called from client side session."] # [doc = " :enum:`NGHTTP2_ERR_INVALID_ARGUMENT`"] # [doc = "     There are too many origins, or an origin is too large to fit"] # [doc = "     into a default frame payload."] pub fn nghttp2_submit_origin (session : * mut nghttp2_session , flags : u8 , ov : * const nghttp2_origin_entry , nov : usize ,) -> :: std :: os :: raw :: c_int ; }
};
}
