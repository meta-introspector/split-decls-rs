// Generated macro for macro_373 (macro)
macro_rules! Depcrate_fut_stream_foldmacro_373 {
() => {
// Module: crate::fut::stream::fold
// Provides: {"macro_373"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`fold`](super::ActorStreamExt::fold) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Fold < S , F , Fut , T > { # [pin] stream : S , f : F , accum : Option < T >, # [pin] future : Option < Fut >, } }
};
}
