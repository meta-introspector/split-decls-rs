// Generated macro for macro_939 (macro)
macro_rules! Depcrate_stream_stream_try_for_eachmacro_939 {
() => {
// Module: crate::stream::stream::try_for_each
// Provides: {"macro_939"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`try_for_each`](super::StreamExt::try_for_each) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryForEach < St , Fut , F > { # [pin] stream : St , f : F , # [pin] future : Option < Fut >, } }
};
}
