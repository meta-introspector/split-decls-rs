// Generated macro for macro_1198 (macro)
macro_rules! Depcrate_stream_try_streammacro_1198 {
() => {
// Module: crate::stream::try_stream
// Provides: {"macro_1198"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`err_into`](super::TryStreamExt::err_into) method."] ErrInto < St , E > (MapErr < St , IntoFn < E >>) : Debug + Sink + Stream + FusedStream + AccessInner [St , (.)] + New [| x : St | MapErr :: new (x , into_fn ())]) ;
};
}
