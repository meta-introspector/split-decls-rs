// Generated macro for macro_703 (macro)
macro_rules! Depcrate_stream_stream_allmacro_703 {
() => {
// Module: crate::stream::stream::all
// Provides: {"macro_703"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`all`](super::StreamExt::all) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct All < St , Fut , F > { # [pin] stream : St , f : F , done : bool , # [pin] future : Option < Fut >, } }
};
}
