// Generated macro for Inner (struct)
macro_rules! Depcrate_easy_handlerInner {
() => {
// Module: crate::easy::handler
// Provides: {"Inner"}
// Dependencies: {}
struct Inner < H > { handle : * mut curl_sys :: CURL , header_list : Option < List > , resolve_list : Option < List > , connect_to_list : Option < List > , form : Option < Form > , error_buf : RefCell < Vec < u8 > > , handler : H , }
};
}
