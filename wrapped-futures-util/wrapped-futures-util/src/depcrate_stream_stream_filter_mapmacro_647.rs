// Generated macro for macro_647 (macro)
macro_rules! Depcrate_stream_stream_filter_mapmacro_647 {
() => {
// Module: crate::stream::stream::filter_map
// Provides: {"macro_647"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`filter_map`](super::StreamExt::filter_map) method."] # [must_use = "streams do nothing unless polled"] pub struct FilterMap < St , Fut , F > { # [pin] stream : St , f : F , # [pin] pending : Option < Fut >, } }
};
}
