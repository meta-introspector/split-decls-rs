// Generated macro for macro_577 (macro)
macro_rules! Depcrate_stream_stream_concatmacro_577 {
() => {
// Module: crate::stream::stream::concat
// Provides: {"macro_577"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`concat`](super::StreamExt::concat) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Concat < St : Stream > { # [pin] stream : St , accum : Option < St :: Item >, } }
};
}
