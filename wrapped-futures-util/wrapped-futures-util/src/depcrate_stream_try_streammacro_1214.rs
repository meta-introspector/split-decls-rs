// Generated macro for macro_1214 (macro)
macro_rules! Depcrate_stream_try_streammacro_1214 {
() => {
// Module: crate::stream::try_stream
// Provides: {"macro_1214"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`map_err`](super::TryStreamExt::map_err) method."] MapErr < St , F > (Map < IntoStream < St >, MapErrFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | Map :: new (IntoStream :: new (x) , map_err_fn (f))]) ;
};
}
