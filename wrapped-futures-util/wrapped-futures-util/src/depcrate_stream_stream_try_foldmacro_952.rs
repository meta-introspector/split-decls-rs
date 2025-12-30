// Generated macro for macro_952 (macro)
macro_rules! Depcrate_stream_stream_try_foldmacro_952 {
() => {
// Module: crate::stream::stream::try_fold
// Provides: {"macro_952"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`try_fold`](super::TryStreamExt::try_fold) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryFold < St , Fut , T , F > { # [pin] stream : St , f : F , accum : Option < T >, # [pin] future : Option < Fut >, } }
};
}
