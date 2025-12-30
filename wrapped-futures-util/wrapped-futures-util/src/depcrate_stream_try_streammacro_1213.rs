// Generated macro for macro_1213 (macro)
macro_rules! Depcrate_stream_try_streammacro_1213 {
() => {
// Module: crate::stream::try_stream
// Provides: {"macro_1213"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`map_ok`](super::TryStreamExt::map_ok) method."] MapOk < St , F > (Map < IntoStream < St >, MapOkFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | Map :: new (IntoStream :: new (x) , map_ok_fn (f))]) ;
};
}
