// Generated macro for octet_stream (function)
macro_rules! Depcrateoctet_stream {
() => {
// Module: crate
// Provides: {"octet_stream"}
// Dependencies: {}
# [doc = " Get the MIME type for `application/octet-stream` (generic binary stream)"] # [deprecated (since = "2.0.0" , note = "use `mime::APPLICATION_OCTET_STREAM` instead")] pub fn octet_stream () -> Mime { "application/octet-stream" . parse () . unwrap () }
};
}
