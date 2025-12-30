// Generated macro for macro_1337 (macro)
macro_rules! Depcrate_stream_try_stream_try_collectmacro_1337 {
() => {
// Module: crate::stream::try_stream::try_collect
// Provides: {"macro_1337"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`try_collect`](super::TryStreamExt::try_collect) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryCollect < St , C > { # [pin] stream : St , items : C , } }
};
}
