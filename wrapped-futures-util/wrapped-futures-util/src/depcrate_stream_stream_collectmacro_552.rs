// Generated macro for macro_552 (macro)
macro_rules! Depcrate_stream_stream_collectmacro_552 {
() => {
// Module: crate::stream::stream::collect
// Provides: {"macro_552"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`collect`](super::StreamExt::collect) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Collect < St , C > { # [pin] stream : St , collection : C , } }
};
}
