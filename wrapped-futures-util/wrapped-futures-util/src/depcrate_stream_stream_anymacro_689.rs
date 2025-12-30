// Generated macro for macro_689 (macro)
macro_rules! Depcrate_stream_stream_anymacro_689 {
() => {
// Module: crate::stream::stream::any
// Provides: {"macro_689"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`any`](super::StreamExt::any) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Any < St , Fut , F > { # [pin] stream : St , f : F , done : bool , # [pin] future : Option < Fut >, } }
};
}
