// Generated macro for macro_357 (macro)
macro_rules! Depcrate_fut_stream_collectmacro_357 {
() => {
// Module: crate::fut::stream::collect
// Provides: {"macro_357"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`collect`](super::ActorStreamExt::collect) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Collect < S , C > { # [pin] stream : S , collection : C , } }
};
}
