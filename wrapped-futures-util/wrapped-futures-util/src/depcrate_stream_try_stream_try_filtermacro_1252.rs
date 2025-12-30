// Generated macro for macro_1252 (macro)
macro_rules! Depcrate_stream_try_stream_try_filtermacro_1252 {
() => {
// Module: crate::stream::try_stream::try_filter
// Provides: {"macro_1252"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_filter`](super::TryStreamExt::try_filter)"] # [doc = " method."] # [must_use = "streams do nothing unless polled"] pub struct TryFilter < St , Fut , F > where St : TryStream { # [pin] stream : St , f : F , # [pin] pending_fut : Option < Fut >, pending_item : Option < St :: Ok >, } }
};
}
