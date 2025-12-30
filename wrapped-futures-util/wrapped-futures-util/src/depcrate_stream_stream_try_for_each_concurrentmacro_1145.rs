// Generated macro for macro_1145 (macro)
macro_rules! Depcrate_stream_stream_try_for_each_concurrentmacro_1145 {
() => {
// Module: crate::stream::stream::try_for_each_concurrent
// Provides: {"macro_1145"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the"] # [doc = " [`try_for_each_concurrent`](super::TryStreamExt::try_for_each_concurrent)"] # [doc = " method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryForEachConcurrent < St , Fut , F > { # [pin] stream : Option < St >, f : F , futures : FuturesUnordered < Fut >, limit : Option < NonZeroUsize >, } }
};
}
