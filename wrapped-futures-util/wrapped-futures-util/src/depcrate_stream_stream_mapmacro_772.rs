// Generated macro for macro_772 (macro)
macro_rules! Depcrate_stream_stream_mapmacro_772 {
() => {
// Module: crate::stream::stream::map
// Provides: {"macro_772"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`map`](super::StreamExt::map) method."] # [must_use = "streams do nothing unless polled"] pub struct Map < St , F > { # [pin] stream : St , f : F , } }
};
}
