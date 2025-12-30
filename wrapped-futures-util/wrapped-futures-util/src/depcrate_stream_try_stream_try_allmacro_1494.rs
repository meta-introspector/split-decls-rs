// Generated macro for macro_1494 (macro)
macro_rules! Depcrate_stream_try_stream_try_allmacro_1494 {
() => {
// Module: crate::stream::try_stream::try_all
// Provides: {"macro_1494"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`try_all`](super::TryStreamExt::try_all) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryAll < St , Fut , F > { # [pin] stream : St , f : F , done : bool , # [pin] future : Option < Fut >, } }
};
}
