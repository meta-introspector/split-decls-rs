// Generated macro for macro_1349 (macro)
macro_rules! Depcrate_stream_try_stream_try_concatmacro_1349 {
() => {
// Module: crate::stream::try_stream::try_concat
// Provides: {"macro_1349"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`try_concat`](super::TryStreamExt::try_concat) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryConcat < St : TryStream > { # [pin] stream : St , accum : Option < St :: Ok >, } }
};
}
