// Generated macro for Headers (struct)
macro_rules! Depcrate_frame_headersHeaders {
() => {
// Module: crate::frame::headers
// Provides: {"Headers"}
// Dependencies: {}
# [doc = " Header frame"] # [doc = ""] # [doc = " This could be either a request or a response."] # [derive (Eq , PartialEq)] pub struct Headers { # [doc = " The ID of the stream with which this frame is associated."] stream_id : StreamId , # [doc = " The stream dependency information, if any."] stream_dep : Option < StreamDependency > , # [doc = " The header block fragment"] header_block : HeaderBlock , # [doc = " The associated flags"] flags : HeadersFlag , }
};
}
