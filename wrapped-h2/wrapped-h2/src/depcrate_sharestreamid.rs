// Generated macro for StreamId (struct)
macro_rules! Depcrate_shareStreamId {
() => {
// Module: crate::share
// Provides: {"StreamId"}
// Dependencies: {}
# [doc = " A stream identifier, as described in [Section 5.1.1] of RFC 7540."] # [doc = ""] # [doc = " Streams are identified with an unsigned 31-bit integer. Streams"] # [doc = " initiated by a client MUST use odd-numbered stream identifiers; those"] # [doc = " initiated by the server MUST use even-numbered stream identifiers.  A"] # [doc = " stream identifier of zero (0x0) is used for connection control"] # [doc = " messages; the stream identifier of zero cannot be used to establish a"] # [doc = " new stream."] # [doc = ""] # [doc = " [Section 5.1.1]: https://tools.ietf.org/html/rfc7540#section-5.1.1"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] pub struct StreamId (u32) ;
};
}
