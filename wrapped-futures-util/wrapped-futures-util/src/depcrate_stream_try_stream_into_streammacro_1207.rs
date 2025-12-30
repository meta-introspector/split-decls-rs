// Generated macro for macro_1207 (macro)
macro_rules! Depcrate_stream_try_stream_into_streammacro_1207 {
() => {
// Module: crate::stream::try_stream::into_stream
// Provides: {"macro_1207"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`into_stream`](super::TryStreamExt::into_stream) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct IntoStream < St > { # [pin] stream : St , } }
};
}
