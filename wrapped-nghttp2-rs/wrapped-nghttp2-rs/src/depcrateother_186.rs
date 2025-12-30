// Generated macro for other_186 (other)
macro_rules! Depcrateother_186 {
() => {
// Module: crate
// Provides: {"other_186"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " RFC 7540 does not enforce any limit on the number of incoming"] # [doc = " reserved streams (in RFC 7540 terms, streams in reserved (remote)"] # [doc = " state).  This only affects client side, since only server can push"] # [doc = " streams.  Malicious server can push arbitrary number of streams,"] # [doc = " and make client's memory exhausted.  This option can set the"] # [doc = " maximum number of such incoming streams to avoid possible memory"] # [doc = " exhaustion.  If this option is set, and pushed streams are"] # [doc = " automatically closed on reception, without calling user provided"] # [doc = " callback, if they exceed the given limit.  The default value is"] # [doc = " 200.  If session is configured as server side, this option has no"] # [doc = " effect.  Server can control the number of streams to push."] pub fn nghttp2_option_set_max_reserved_remote_streams (option : * mut nghttp2_option , val : u32) ; }
};
}
