// Generated macro for macro_114 (macro)
macro_rules! Depcrate_future_futuremacro_114 {
() => {
// Module: crate::future::future
// Provides: {"macro_114"}
// Dependencies: {}
delegate_all ! (# [doc = " Stream for the [`flatten_stream`](FutureExt::flatten_stream) method."] FlattenStream < F > (flatten :: Flatten < F , < F as Future >:: Output >) : Debug + Sink + Stream + FusedStream + New [| x : F | flatten :: Flatten :: new (x)] where F : Future) ;
};
}
