// Generated macro for other_240 (other)
macro_rules! Depcrateother_240 {
() => {
// Module: crate
// Provides: {"other_240"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " Initializes |pri_spec| with the |stream_id| of the stream to depend"] # [doc = " on with |weight| and its exclusive flag.  If |exclusive| is"] # [doc = " nonzero, exclusive flag is set."] # [doc = ""] # [doc = " The |weight| must be in [:enum:`NGHTTP2_MIN_WEIGHT`,"] # [doc = " :enum:`NGHTTP2_MAX_WEIGHT`], inclusive."] pub fn nghttp2_priority_spec_init (pri_spec : * mut nghttp2_priority_spec , stream_id : i32 , weight : i32 , exclusive : :: std :: os :: raw :: c_int ,) ; }
};
}
