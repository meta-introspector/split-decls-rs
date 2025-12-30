// Generated macro for JsStream (struct)
macro_rules! Depcrate_streamJsStream {
() => {
// Module: crate::stream
// Provides: {"JsStream"}
// Dependencies: {}
# [doc = " A `Stream` that yields values from an underlying `AsyncIterator`."] pub struct JsStream { iter : AsyncIterator , next : Option < JsFuture > , done : bool , }
};
}
