// Generated macro for macro_590 (macro)
macro_rules! Depcrate_stream_stream_countmacro_590 {
() => {
// Module: crate::stream::stream::count
// Provides: {"macro_590"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`count`](super::StreamExt::count) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Count < St > { # [pin] stream : St , count : usize } }
};
}
