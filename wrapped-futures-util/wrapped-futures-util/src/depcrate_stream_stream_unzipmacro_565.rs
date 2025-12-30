// Generated macro for macro_565 (macro)
macro_rules! Depcrate_stream_stream_unzipmacro_565 {
() => {
// Module: crate::stream::stream::unzip
// Provides: {"macro_565"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`unzip`](super::StreamExt::unzip) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Unzip < St , FromA , FromB > { # [pin] stream : St , left : FromA , right : FromB , } }
};
}
