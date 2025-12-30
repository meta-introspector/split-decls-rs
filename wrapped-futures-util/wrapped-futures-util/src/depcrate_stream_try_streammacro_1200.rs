// Generated macro for macro_1200 (macro)
macro_rules! Depcrate_stream_try_streammacro_1200 {
() => {
// Module: crate::stream::try_stream
// Provides: {"macro_1200"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`inspect_err`](super::TryStreamExt::inspect_err) method."] InspectErr < St , F > (Inspect < IntoStream < St >, InspectErrFn < F >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (. .)] + New [| x : St , f : F | Inspect :: new (IntoStream :: new (x) , inspect_err_fn (f))]) ;
};
}
