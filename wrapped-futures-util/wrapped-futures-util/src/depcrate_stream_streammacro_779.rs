// Generated macro for macro_779 (macro)
macro_rules! Depcrate_stream_streammacro_779 {
() => {
// Module: crate::stream::stream
// Provides: {"macro_779"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`flat_map`](StreamExt::flat_map) method."] FlatMap < St , U , F > (flatten :: Flatten < Map < St , F >, U >) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | flatten :: Flatten :: new (Map :: new (x , f))]) ;
};
}
