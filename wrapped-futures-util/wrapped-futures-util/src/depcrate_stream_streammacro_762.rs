// Generated macro for macro_762 (macro)
macro_rules! Depcrate_stream_streammacro_762 {
() => {
// Module: crate::stream::stream
// Provides: {"macro_762"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`inspect`](StreamExt::inspect) method."] Inspect < St , F > (map :: Map < St , InspectFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (.)] + New [| x : St , f : F | map :: Map :: new (x , inspect_fn (f))]) ;
};
}
