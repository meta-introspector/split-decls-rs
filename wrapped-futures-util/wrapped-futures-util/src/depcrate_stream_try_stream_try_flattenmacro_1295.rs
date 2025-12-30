// Generated macro for macro_1295 (macro)
macro_rules! Depcrate_stream_try_stream_try_flattenmacro_1295 {
() => {
// Module: crate::stream::try_stream::try_flatten
// Provides: {"macro_1295"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`try_flatten`](super::TryStreamExt::try_flatten) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct TryFlatten < St > where St : TryStream , { # [pin] stream : St , # [pin] next : Option < St :: Ok >, } }
};
}
