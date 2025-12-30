// Generated macro for BasicClient (struct)
macro_rules! Depcrate_basicBasicClient {
() => {
// Module: crate::basic
// Provides: {"BasicClient"}
// Dependencies: {}
# [doc = " Client for a `Basic` challenge, as in"] # [doc = " [RFC 7617](https://datatracker.ietf.org/doc/html/rfc7617)."] # [doc = ""] # [doc = " This implementation always uses `UTF-8`. Thus it doesn't use or store the"] # [doc = " `charset` parameter, which the RFC only allows to be set to `UTF-8` anyway."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct BasicClient { realm : Box < str > , }
};
}
