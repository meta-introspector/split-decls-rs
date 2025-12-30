// Generated macro for HeaderBlock (struct)
macro_rules! Depcrate_frame_headersHeaderBlock {
() => {
// Module: crate::frame::headers
// Provides: {"HeaderBlock"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq)] struct HeaderBlock { # [doc = " The decoded header fields"] fields : HeaderMap , # [doc = " Precomputed size of all of our header fields, for perf reasons"] field_size : usize , # [doc = " Set to true if decoding went over the max header list size."] is_over_size : bool , # [doc = " Pseudo headers, these are broken out as they must be sent as part of the"] # [doc = " headers frame."] pseudo : Pseudo , }
};
}
