// Generated macro for macro_1199 (macro)
macro_rules! Depcrate_stream_try_streammacro_1199 {
() => {
// Module: crate::stream::try_stream
// Provides: {"macro_1199"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`inspect_ok`](super::TryStreamExt::inspect_ok) method."] InspectOk < St , F > (Inspect < IntoStream < St >, InspectOkFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | Inspect :: new (IntoStream :: new (x) , inspect_ok_fn (f))]) ;
};
}
