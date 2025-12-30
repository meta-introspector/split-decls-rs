// Generated macro for Reason (struct)
macro_rules! Depcrate_frame_reasonReason {
() => {
// Module: crate::frame::reason
// Provides: {"Reason"}
// Dependencies: {}
# [doc = " HTTP/2 error codes."] # [doc = ""] # [doc = " Error codes are used in `RST_STREAM` and `GOAWAY` frames to convey the"] # [doc = " reasons for the stream or connection error. For example,"] # [doc = " [`SendStream::send_reset`] takes a `Reason` argument. Also, the `Error` type"] # [doc = " may contain a `Reason`."] # [doc = ""] # [doc = " Error codes share a common code space. Some error codes apply only to"] # [doc = " streams, others apply only to connections, and others may apply to either."] # [doc = " See [RFC 7540] for more information."] # [doc = ""] # [doc = " See [Error Codes in the spec][spec]."] # [doc = ""] # [doc = " [spec]: http://httpwg.org/specs/rfc7540.html#ErrorCodes"] # [doc = " [`SendStream::send_reset`]: struct.SendStream.html#method.send_reset"] # [derive (PartialEq , Eq , Clone , Copy)] pub struct Reason (u32) ;
};
}
