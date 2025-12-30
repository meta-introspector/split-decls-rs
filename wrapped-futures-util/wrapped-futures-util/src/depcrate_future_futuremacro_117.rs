// Generated macro for macro_117 (macro)
macro_rules! Depcrate_future_futuremacro_117 {
() => {
// Module: crate::future::future
// Provides: {"macro_117"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`into_stream`](FutureExt::into_stream) method."] IntoStream < F > (crate :: stream :: Once < F >) : Debug + Stream + FusedStream + New [| x : F | crate :: stream :: Once :: new (x)]) ;
};
}
