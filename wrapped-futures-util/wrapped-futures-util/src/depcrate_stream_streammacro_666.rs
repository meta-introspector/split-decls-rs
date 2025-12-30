// Generated macro for macro_666 (macro)
macro_rules! Depcrate_stream_streammacro_666 {
() => {
// Module: crate::stream::stream
// Provides: {"macro_666"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`flatten`](StreamExt::flatten) method."] Flatten < St > (flatten :: Flatten < St , St :: Item >) : Debug + Sink + Stream + FusedStream + AccessInner [St , (.)] + New [| x : St | flatten :: Flatten :: new (x)] where St : Stream) ;
};
}
