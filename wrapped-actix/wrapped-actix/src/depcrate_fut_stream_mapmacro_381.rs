// Generated macro for macro_381 (macro)
macro_rules! Depcrate_fut_stream_mapmacro_381 {
() => {
// Module: crate::fut::stream::map
// Provides: {"macro_381"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`map`](super::ActorStreamExt::map) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Map < S , F > { # [pin] stream : S , f : F , } }
};
}
