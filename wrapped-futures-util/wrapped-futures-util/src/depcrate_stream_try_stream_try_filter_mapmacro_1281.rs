// Generated macro for macro_1281 (macro)
macro_rules! Depcrate_stream_try_stream_try_filter_mapmacro_1281 {
() => {
// Module: crate::stream::try_stream::try_filter_map
// Provides: {"macro_1281"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_filter_map`](super::TryStreamExt::try_filter_map)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct TryFilterMap < St , Fut , F > { # [pin] stream : St , f : F , # [pin] pending : Option < Fut >, } }
};
}
