// Generated macro for macro_1103 (macro)
macro_rules! Depcrate_stream_stream_for_each_concurrentmacro_1103 {
() => {
// Module: crate::stream::stream::for_each_concurrent
// Provides: {"macro_1103"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`for_each_concurrent`](super::StreamExt::for_each_concurrent)"] # [doc = " method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ForEachConcurrent < St , Fut , F > { # [pin] stream : Option < St >, f : F , futures : FuturesUnordered < Fut >, limit : Option < NonZeroUsize >, } }
};
}
